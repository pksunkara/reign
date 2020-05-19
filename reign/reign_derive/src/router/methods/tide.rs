use crate::router::Methods;
use proc_macro2::TokenStream;
use quote::quote;

pub fn tide(input: Methods) -> TokenStream {
    let Methods {
        methods,
        path,
        action,
    } = input;

    let methods = methods.iter().map(|i| i).collect::<Vec<_>>();

    path.tide(false)
        .iter()
        .map(|path| {
            quote! {
                app.at(#path)
                    #(.#methods(#action))*
            }
        })
        .collect()
}
