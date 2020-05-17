use crate::router::scope::Scope;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

pub fn gotham(input: Scope) -> TokenStream {
    let Scope { path, pipe, rest } = input;

    let pipes = if let Some(pipe) = pipe {
        let mut chains = vec![];
        let mut iter = pipe.into_iter().map(|i| i);
        let mut prev = None;

        while let Some(i) = iter.next() {
            let name = Ident::new(&format!("{}_pipe", i), Span::call_site());
            let chain = Ident::new(&format!("{}_chain", i), Span::call_site());

            if let Some(inside) = prev {
                let prev_chain = Ident::new(&format!("{}_chain", inside), Span::call_site());

                chains.push(quote! {
                    let #chain = (#name, #prev_chain);
                });
            } else {
                chains.push(quote! {
                    let #chain = (#name, ());
                });
            }

            prev = Some(i);
        }

        let chain = if let Some(inside) = prev {
            let prev_chain = Ident::new(&format!("{}_chain", inside), inside.span());

            quote! {
                #prev_chain
            }
        } else {
            quote! {
                ()
            }
        };

        quote! {
            {
                #(#chains)*
                #chain
            }
        }
    } else {
        quote! {
            ()
        }
    };

    quote! {
        route
            .delegate(#path)
            .to_router(::gotham::router::builder::build_router(
                #pipes,
                pipeline_set.clone(),
                |route| {
                    #rest
                }
            ))
    }
}
