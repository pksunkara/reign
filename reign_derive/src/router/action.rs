use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemFn;

pub fn action(input: ItemFn) -> TokenStream {
    let ItemFn {
        attrs, sig, block, ..
    } = input;
    let name = sig.ident;

    if cfg!(feature = "router-actix") {
        quote! {
            #(#attrs)*
            pub async fn #name(
                req: ::actix_web::HttpRequest,
            ) -> ::actix_web::HttpResponse {
                use ::actix_web::Responder;

                async fn _call(
                    req: &::actix_web::HttpRequest,
                ) -> Result<impl Responder, crate::errors::Error> #block

                let _called = _call(&req).await;

                match _called {
                    Ok(r) => match r.respond_to(&req).await {
                        Ok(r) => r,
                        Err(e) => ::actix_web::HttpResponse::from_error(e.into()),
                    },
                    Err(e) => {
                        ::reign::log::error!("{}", e);
                        e.respond()
                    },
                }
            }
        }
    } else if cfg!(feature = "router-gotham") {
        quote! {
            #(#attrs)*
            pub fn #name(
                mut state: ::gotham::state::State,
            ) -> std::pin::Pin<Box<::gotham::handler::HandlerFuture>> {
                use ::futures::prelude::*;
                use ::gotham::handler::IntoResponse;

                async fn _call(
                    state: &mut ::gotham::state::State,
                ) -> Result<impl IntoResponse, crate::errors::Error> #block

                async move {
                    let _called = _call(&mut state).await;

                    match _called {
                        Ok(r) => {
                            let r = r.into_response(&state);
                            Ok((state, r))
                        },
                        Err(e) => {
                            ::reign::log::error!("{}", e);
                            Ok((state, e.respond()))
                        },
                    }
                }.boxed()
            }
        }
    } else if cfg!(feature = "router-tide") {
        quote! {
            #(#attrs)*
            pub async fn #name(
                req: ::tide::Request<()>,
            ) -> ::tide::Response {
                use ::tide::IntoResponse;

                async fn _call(
                    req: ::tide::Request<()>,
                ) -> Result<impl IntoResponse, crate::errors::Error> #block

                let _called = _call(req).await;

                match _called {
                    Ok(r) => r.into_response(),
                    Err(e) => {
                        ::reign::log::error!("{}", e);
                        e.respond()
                    },
                }
            }
        }
    } else {
        quote! {}
    }
}
