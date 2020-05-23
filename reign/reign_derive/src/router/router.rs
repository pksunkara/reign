use crate::router::ty::only_one;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Expr, ItemFn, Stmt};

pub fn router(input: ItemFn) -> TokenStream {
    let ItemFn {
        attrs, sig, block, ..
    } = input;

    let name = sig.ident;
    let mut saw_scope = false;
    let stmts = block.stmts;

    let (scopes, pipes) = stmts.into_iter().partition::<Vec<_>, _>(move |stmt| {
        match stmt {
            Stmt::Expr(Expr::Macro(e)) | Stmt::Semi(Expr::Macro(e), _) => {
                if let Some(name) = only_one(e.mac.path.segments.iter()) {
                    if name.ident == "scope" {
                        saw_scope = true;
                    }
                }
            }
            _ => {}
        }

        return saw_scope;
    });

    // TODO:(router) No need for macros

    if cfg!(feature = "router-actix") {
        quote! {
            #(#attrs)*
            pub async fn #name<A>(addr: A) -> std::io::Result<()>
            where
                A: std::net::ToSocketAddrs + Send + 'static
            {
                ::actix_web::HttpServer::new(|| {
                    let mut app = ::actix_web::App::new();

                    #(#pipes)*
                    #(#scopes)*

                    app
                })
                .bind(addr)
                .unwrap()
                .run()
                .await
            }
        }
    } else if cfg!(feature = "router-gotham") {
        quote! {
            #(#attrs)*
            pub async fn #name<A>(addr: A) -> Result<(), ()>
            where
                A: std::net::ToSocketAddrs + Send + 'static
            {
                use ::gotham::router::builder::{DrawRoutes, DefineSingleRoute};

                ::gotham::init_server(
                    addr,
                    ::gotham::router::builder::build_simple_router(|route| {
                        let pipelines = ::gotham::pipeline::set::new_pipeline_set();

                        #(#pipes)*

                        let pipeline_set = ::gotham::pipeline::set::finalize_pipeline_set(pipelines);

                        route.delegate("").to_router(
                            ::gotham::router::builder::build_router((), pipeline_set, |route| {
                                let __chain = ();

                                #(#scopes)*
                            })
                        );
                    })
                ).await
            }
        }
    } else if cfg!(feature = "router-tide") {
        quote! {
            #(#attrs)*
            pub async fn #name<A>(addr: A) -> std::io::Result<()>
            where
                A: ::async_std::net::ToSocketAddrs + 'static
            {
                let mut app = ::tide::new();

                #(#pipes)*
                #(#scopes)*

                app.listen(addr).await
            }
        }
    } else {
        quote! {
            #(#attrs)*
            pub async fn #name<A>(addr: A) -> std::io::Result<()>
            where
                A: ::async_std::net::ToSocketAddrs + 'static
            {
                #(#pipes)*
                #(#scopes)*
            }
        }
    }
}
