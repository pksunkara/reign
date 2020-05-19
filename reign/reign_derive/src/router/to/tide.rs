use crate::router::To;
use inflector::cases::snakecase::to_snake_case;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub fn tide(input: To) -> TokenStream {
    let To {
        methods,
        path,
        action,
        prev,
    } = input;

    let (paths, params) = path.tide(false);
    let methods = methods
        .iter()
        .map(|i| Ident::new(&to_snake_case(&i.to_string()), i.span()))
        .collect::<Vec<_>>();

    paths
        .iter()
        .map(|path| {
            quote! {
                app.at(#path)
                    #(.#methods(#action))*
            }
        })
        .collect()
}
