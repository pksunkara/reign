use crate::router::path::Path;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    bracketed,
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    token::{Bracket, Comma},
    Ident, Path as SynPath,
};

mod actix;
mod gotham;
mod tide;

pub struct To {
    methods: Punctuated<Ident, Comma>,
    path: Path,
    action: SynPath,
    prev: Option<Path>,
}

impl Parse for To {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(To {
            methods: {
                if input.peek(Bracket) {
                    let content;
                    bracketed!(content in input);
                    content.parse_terminated(|i| i.parse::<Ident>())?
                } else {
                    let mut methods = Punctuated::new();
                    methods.push(input.parse()?);
                    methods
                }
                //TODO:(router) Unallowed methods
            },
            path: {
                input.parse::<Comma>()?;
                input.parse()?
            },
            action: {
                input.parse::<Comma>()?;
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

pub fn to(input: To) -> TokenStream {
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
