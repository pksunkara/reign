use crate::router::{path::Path, Scope};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

fn gen_path(path: Path) -> TokenStream {
    quote! {}
}

pub fn tide(input: Scope) -> TokenStream {
    let Scope { path, pipe, rest } = input;

    let pipes = if let Some(pipe) = pipe {
        pipe.into_iter()
            .map(|i| {
                let name = Ident::new(&format!("{}_pipe", i), Span::call_site());

                quote! {
                    #name(&mut app);
                }
            })
            .collect()
    } else {
        vec![]
    };

    let path = gen_path(path);

    quote! {
        app.at(#path).nest({
            let mut app = ::tide::new();

            #(#pipes)*
            #rest

            app
        })
    }
}
