use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemFn;

pub fn read_form_attribute(item: ItemFn) -> TokenStream {
    let ItemFn {
        attrs, sig, block, ..
    } = item;
    let stmts = (*block).stmts;

    // TODO: No need for return type
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
                        let body = valid_body.into_bytes();
                        let form_data = form_urlencoded::parse(&body).into_owned();

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
