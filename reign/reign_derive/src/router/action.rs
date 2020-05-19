use proc_macro2::TokenStream;
use quote::quote;
use syn::{ItemFn, Signature};

pub fn action(input: ItemFn) -> TokenStream {
    let ItemFn {
        attrs, sig, block, ..
    } = input;
    let Signature { ident, inputs, .. } = sig;

    if cfg!(feature = "router-actix") {
        quote! {
            #(#attrs)*
            pub async fn #ident(
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
            pub async fn #ident(
                state: &mut ::gotham::state::State,
                #inputs
            ) -> Result<impl ::gotham::handler::IntoResponse, crate::errors::Error> #block
        }
    } else if cfg!(feature = "router-tide") {
        quote! {
            #(#attrs)*
            pub async fn #ident(
                req: ::tide::Request<()>,
            ) -> ::tide::Result<::tide::Response> {
                async fn _call(
                    req: ::tide::Request<()>,
                ) -> Result<impl Into<::tide::Response>, crate::errors::Error> #block

                let _called = _call(req).await;

                let response = match _called {
                    Ok(r) => r.into(),
                    Err(e) => {
                        ::reign::log::error!("{}", e);
                        e.respond()
                    },
                };

                Ok(response)
            }
        }
    } else {
        quote! {}
    }
}
