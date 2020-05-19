use crate::router::Scope;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

pub fn gotham(input: Scope) -> TokenStream {
    let Scope { path, pipe, rest } = input;

    let pipes = if let Some(pipe) = pipe {
        let mut chains = vec![];
        let mut iter = pipe.into_iter().map(|i| i);
        let mut prev = Ident::new("__chain", Span::call_site());

        while let Some(i) = iter.next() {
            let name = Ident::new(&format!("{}_pipe", i), i.span());
            let chain = Ident::new(&format!("{}_chain", i), i.span());

            chains.push(quote! {
                let #chain = (#name, #prev);
            });

            prev = chain;
        }

        quote! {
            #(#chains)*
            let __chain = #prev
        }
    } else {
        quote! {}
    };

    path.gotham(true)
        .iter()
        .map(|path| {
            quote! {
                route.scope(#path, |route| {
                    #pipes;

                    route.with_pipeline_chain(__chain, |route| {
                        #rest
                    });
                })
            }
        })
        .collect()
}
