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
            ) -> impl ::actix_web::Responder {
                async fn _call(
                    req: ::actix_web::HttpRequest,
                ) -> Result<impl ::actix_web::Responder, crate::errors::Error> #block

                let _called = _call(req).await;

                match _called {
                    Ok(r) => r,
                    Err(e) => e.respond(),
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

                async fn _call(
                    state: &mut State,
                ) -> Result<::gotham::hyper::Response<::gotham::hyper::Body>, crate::errors::Error> #block

                async move {
                    let _called = _call(&mut state).await;

                    match _called {
                        Ok(r) => Ok((state, r)),
                        Err(e) => Ok((state, e.respond())),
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
                async fn _call(
                    req: ::tide::Request<()>
                ) -> Result<::tide::Response, crate::errors::Error> #block

                let _called = _call(req).await;

                match _called {
                    Ok(r) => r,
                    Err(e) => e.respond(),
                }
            }
        }
    } else {
        quote! {}
    }
}
