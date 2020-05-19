use crate::router::path::Path;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    braced, bracketed,
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    token::{Bracket, Comma},
    Ident,
};

mod actix;
mod gotham;
mod tide;

pub struct Scope {
    path: Path,
    pipe: Option<Punctuated<Ident, Comma>>,
    rest: TokenStream,
}

impl Parse for Scope {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut content;

        Ok(Scope {
            path: input.parse()?,
            pipe: {
                if input.peek(Comma) {
                    input.parse::<Comma>()?;
                }

                if input.peek(Bracket) {
                    bracketed!(content in input);
                    Some(content.parse_terminated(|i| i.parse::<Ident>())?)
                } else {
                    None
                }
            },
            rest: {
                if input.peek(Comma) {
                    input.parse::<Comma>()?;
                }

                braced!(content in input);
                content.parse()?
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
