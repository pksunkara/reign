use crate::router::ty::only_one;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Expr, ItemFn, Stmt};

pub fn router(input: ItemFn) -> TokenStream {
    let ItemFn {
        attrs, sig, block, ..
    } = input;

    let name = sig.ident;
    let mut saw_pipelines = false;
    let stmts = block.stmts;

    let (scopes, pipes) = stmts.into_iter().partition::<Vec<_>, _>(move |stmt| {
        let ret = saw_pipelines;

        match stmt {
            Stmt::Expr(Expr::Macro(e)) | Stmt::Semi(Expr::Macro(e), _) => {
                if let Some(name) = only_one(e.mac.path.segments.iter()) {
                    if name.ident == "pipelines" {
                        saw_pipelines = true;
                    }
                }
            }
            _ => {}
        }

        return ret;
    });

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
                use ::gotham::router::builder::*;

                ::gotham::init_server(
                    addr,
                    build_simple_router(|route| {
                        #(#pipes)*

                        route.delegate("").to_router(build_router((), pipeline_set, |route| {
                            let __chain = ();

                            #(#scopes)*
                        }));
                    })
                ).await
            }
        }
    } else if cfg!(feature = "router-tide") {
        quote! {
            #(#attrs)*
            pub async fn #name<A>(addr: A) -> std::io::Result<()>
            where
                A: std::net::ToSocketAddrs + Send + 'static
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
