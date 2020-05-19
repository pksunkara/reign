use crate::router::Methods;
use inflector::cases::screamingsnakecase::to_screaming_snake_case;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub fn gotham(input: Methods) -> TokenStream {
    let Methods {
        methods,
        path,
        action,
    } = input;

    let methods = methods
        .iter()
        .map(|i| Ident::new(&to_screaming_snake_case(&i.to_string()), i.span()))
        .collect::<Vec<_>>();

    path.gotham(false)
        .iter()
        .map(|path| {
            quote! {
                route.request(vec![#(::gotham::hyper::Method::#methods),*], #path).to(#action)
            }
        })
        .collect()
}
