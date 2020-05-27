use crate::router::ty::only_one;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Expr, ItemFn, Stmt};

pub fn router(input: ItemFn) -> TokenStream {
    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = input;

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

        saw_scope
    });

    // TODO:(router) No need for macros

    if cfg!(feature = "router-actix") {
        quote! {
            #(#attrs)*
            #vis #sig
            {
                fn configure(config: &mut ::actix_web::web::ServiceConfig) {
                    config.service({
                        let mut app = actix_web::web::scope("");

                        #(#pipes)*
                        #(#scopes)*

                        app
                    });
                }

                ::reign::router::Router::Actix(configure)
            }
        }
    } else if cfg!(feature = "router-gotham") {
        quote! {
            #(#attrs)*
            #vis #sig
            {
                use ::gotham::router::builder::{DrawRoutes, DefineSingleRoute};
                use ::reign::router::Router::Gotham;

                Gotham(::gotham::router::builder::build_simple_router(|route| {
                    let pipelines = ::gotham::pipeline::set::new_pipeline_set();

                    #(#pipes)*

                    let pipeline_set = ::gotham::pipeline::set::finalize_pipeline_set(pipelines);

                    route.delegate("").to_router(
                        ::gotham::router::builder::build_router((), pipeline_set, |route| {
                            let __chain = ();

                            #(#scopes)*
                        })
                    );
                }))
            }
        }
    } else if cfg!(feature = "router-tide") {
        quote! {
            #(#attrs)*
            #vis #sig
            {
                let mut app = ::tide::new();

                #(#pipes)*
                #(#scopes)*

                ::reign::router::Router::Tide(app)
            }
        }
    } else {
        quote! {}
    }
}
