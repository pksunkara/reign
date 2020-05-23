use crate::router::To;
use inflector::cases::screamingsnakecase::to_screaming_snake_case;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub fn actix(input: To) -> TokenStream {
    let To {
        methods,
        path,
        action,
        prev: _prev,
    } = input;

    let (paths, _params) = path.actix(false);
    let methods = methods
        .iter()
        .map(|i| Ident::new(&to_screaming_snake_case(&i.to_string()), i.span()))
        .collect::<Vec<_>>();

    paths
        .iter()
        .map(|path| {
            quote! {
                app = app
                    #(.route(#path, ::actix_web::web::method(::actix_web::http::Method::#methods).to(#action)))*
            }
        })
        .collect()
}
