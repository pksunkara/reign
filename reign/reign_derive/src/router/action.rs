use proc_macro2::TokenStream;
use quote::quote;
use syn::{FnArg, ItemFn, Pat, Signature};

pub fn action(input: ItemFn) -> TokenStream {
    let ItemFn {
        attrs,
        sig,
        block,
        vis,
    } = input;
    let Signature {
        ident,
        inputs,
        constness,
        asyncness,
        unsafety,
        fn_token,
        // TODO:(router) Use the output directly
        ..
    } = sig;

    let args = inputs
        .iter()
        .flat_map(|x| {
            if let FnArg::Typed(x) = x {
                if let Pat::Ident(x) = &*x.pat {
                    return Some(x.ident.clone());
                }
            }

            None
        })
        .collect::<Vec<_>>();

    if cfg!(feature = "router-actix") {
        quote! {
            #(#attrs)*
            #vis #constness #asyncness #unsafety #fn_token #ident(
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
            #vis #constness #asyncness #unsafety #fn_token #ident(
                state: &mut ::gotham::state::State,
                #inputs
            ) -> ::gotham::hyper::Response<::gotham::hyper::Body> {
                use ::gotham::handler::IntoResponse;

                #[inline]
                async fn _call(
                    state: &mut ::gotham::state::State,
                    #inputs
                ) -> Result<impl IntoResponse, crate::errors::Error> #block

                let _called = _call(state, #(#args),*).await;

                match _called {
                    Ok(r) => r.into_response(&state),
                    Err(e) => {
                        ::reign::log::error!("{}", e);
                        e.respond()
                    },
                }
            }
        }
    } else if cfg!(feature = "router-tide") {
        quote! {
            #(#attrs)*
            #vis #constness #asyncness #unsafety #fn_token #ident(
                req: ::tide::Request<()>,
            ) -> ::tide::Result<::tide::Response> {
                #[inline]
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
