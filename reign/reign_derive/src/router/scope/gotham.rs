use crate::router::{
    path::{Path, PathSegment},
    Scope,
};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

fn gen_path(path: Path) -> String {
    path.segments
        .iter()
        .map(|x| match x {
            PathSegment::Static(s) => s.value(),
            PathSegment::Dynamic(d) => "".to_string(),
        })
        .collect::<Vec<_>>()
        .join("/")
}

pub fn gotham(input: Scope) -> TokenStream {
    let Scope { path, pipe, rest } = input;

    let path = gen_path(path);

    let pipes = if let Some(pipe) = pipe {
        let mut chains = vec![];
        let mut iter = pipe.into_iter().map(|i| i);
        let mut prev = Ident::new("__chain", Span::call_site());

        while let Some(i) = iter.next() {
            let name = Ident::new(&format!("{}_pipe", i), Span::call_site());
            let chain = Ident::new(&format!("{}_chain", i), Span::call_site());

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

    quote! {
        route.scope(#path, |route| {
            #pipes;

            route.with_pipeline_chain(__chain, |route| {
                #rest
            });
        })
    }
}
