use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{
    braced, bracketed,
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    token::{Bracket, Comma},
    Ident, LitStr,
};

mod actix;
mod gotham;
mod tide;
mod warp;

mod path;
mod ty;

use path::Path;

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
