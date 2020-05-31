use crate::util::Stmts;
use proc_macro2::TokenStream;
use quote::quote;

pub fn read_form(input: Stmts) -> TokenStream {
    let Stmts { stmts, .. } = input;

    // TODO: No need for return type in sig
    quote! {
        let f = Body::take_from(&mut state)
            .concat2()
            .then(|full_body| match full_body {
                Ok(valid_body) => {
                    let body = valid_body.into_bytes();
                    let form_data = ::url::form_urlencoded::parse(&body).into_owned();

                    ::futures::future::ok({
                        #(#stmts)*
                    })
                }
                Err(e) => ::futures::future::err((state, e.into_handler_error())),
            });

        Box::new(f)
    }
}
