use crate::router::path::Path;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    bracketed,
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    token::{Bracket, Comma},
    Block, Ident,
};

mod actix;
mod gotham;
mod tide;

pub struct Scope {
    path: Path,
    pipe: Punctuated<Ident, Comma>,
    block: Block,
    prev: Option<Path>,
}

impl Parse for Scope {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Scope {
            path: input.parse()?,
            pipe: {
                if input.peek(Comma) {
                    input.parse::<Comma>()?;
                }

                if input.peek(Bracket) {
                    let content;
                    bracketed!(content in input);
                    content.parse_terminated(|i| i.parse::<Ident>())?
                } else {
                    Punctuated::new()
                }
            },
            block: {
                if input.peek(Comma) {
                    input.parse::<Comma>()?;
                }

                input.parse()?
            },
            prev: {
                if input.peek(Comma) {
                    input.parse::<Comma>()?;
                    Some(input.parse()?)
                } else {
                    None
                }
            },
        })
    }
}

pub fn scope(input: Scope) -> TokenStream {
    if cfg!(feature = "router-actix") {
        actix::actix(input)
    } else if cfg!(feature = "router-gotham") {
        gotham::gotham(input)
    } else if cfg!(feature = "router-tide") {
        tide::tide(input)
    } else {
        quote! {}
    }
}
