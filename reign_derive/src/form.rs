use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemFn;

pub fn form_attribute(item: ItemFn) -> TokenStream {
    let ItemFn {
        attrs, sig, block, ..
    } = item;
    let stmts = (*block).stmts;

    quote! {
        use ::gotham::{handler::IntoHandlerError, state::FromState};
        use ::url::form_urlencoded;
        use ::futures::{future, Future, stream::Stream};

        #(#attrs)*
        pub #sig {
            let f = Body::take_from(&mut state)
                .concat2()
                .then(|full_body| match full_body {
                    Ok(valid_body) => {
                        let body_content = valid_body.into_bytes();
                        let form_data = form_urlencoded::parse(&body_content).into_owned();

                        future::ok({
                            #(#stmts)*
                        })
                    }
                    Err(e) => future::err((state, e.into_handler_error())),
                });

            Box::new(f)
        }
    }
}
