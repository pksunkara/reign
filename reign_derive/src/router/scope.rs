use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    braced, bracketed,
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    token::{Bracket, Comma},
    Ident, LitStr,
};

pub struct Scope {
    path: LitStr,
    pipe: Option<Punctuated<Ident, Comma>>,
    rest: TokenStream,
}

impl Parse for Scope {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut content;

        Ok(Scope {
            path: input.parse()?,
            pipe: {
                if input.peek2(Bracket) {
                    input.parse::<Comma>()?;
                    bracketed!(content in input);
                    Some(content.parse_terminated(|i| i.parse::<Ident>())?)
                } else {
                    None
                }
            },
            rest: {
                input.parse::<Comma>()?;
                braced!(content in input);
                content.parse()?
            },
        })
    }
}

fn chains(input: Punctuated<Ident, Comma>) -> TokenStream {
    let mut chains = vec![];
    let mut iter = input.into_iter().map(|i| i);
    let mut prev = None;

    while let Some(i) = iter.next() {
        let name = Ident::new(&format!("{}_pipe", i), Span::call_site());
        let chain = Ident::new(&format!("{}_chain", i), Span::call_site());

        if prev.is_none() {
            chains.push(quote! {
                let #chain = (#name, ());
            });
        } else {
            let prev_chain = Ident::new(&format!("{}_chain", prev.unwrap()), Span::call_site());

            chains.push(quote! {
                let #chain = (#name, #prev_chain);
            });
        }

        prev = Some(i);
    }

    let chain = if prev.is_some() {
        let prev_chain = Ident::new(&format!("{}_chain", prev.unwrap()), Span::call_site());

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
}

pub fn scope(input: Scope) -> TokenStream {
    let Scope { path, pipe, rest } = input;

    let pipe_tokens = if pipe.is_none() {
        quote! {
            ()
        }
    } else {
        chains(pipe.unwrap())
    };

    quote! {
        route
            .delegate(#path)
            .to_router(::gotham::router::builder::build_router(
                #pipe_tokens,
                pipeline_set.clone(),
                |route| {
                    #rest
                }
            ));
    }
}
