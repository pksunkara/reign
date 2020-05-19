use crate::router::Scope;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

pub fn tide(input: Scope) -> TokenStream {
    let Scope { path, pipe, rest } = input;

    let pipes = if let Some(pipe) = pipe {
        pipe.into_iter()
            .map(|i| {
                let name = Ident::new(&format!("{}_pipe", i), i.span());

                quote! {
                    #name(&mut app);
                }
            })
            .collect()
    } else {
        vec![]
    };

    path.tide(true)
        .iter()
        .map(|path| {
            quote! {
                app.at(#path).nest({
                    let mut app = ::tide::new();

                    #(#pipes)*
                    #rest

                    app
                })
            }
        })
        .collect()
}
