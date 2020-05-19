use crate::router::path::Path;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    bracketed,
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    token::Comma,
    Ident, Path as SynPath,
};

mod actix;
mod gotham;
mod tide;

pub struct Methods {
    methods: Punctuated<Ident, Comma>,
    path: Path,
    action: SynPath,
}

impl Parse for Methods {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;

        Ok(Methods {
            methods: {
                bracketed!(content in input);
                content.parse_terminated(|i| i.parse::<Ident>())?
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
        })
    }
}

pub fn methods(input: Methods) -> TokenStream {
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

pub fn get(input: TokenStream) -> TokenStream {
    quote! {
        methods!([get], #input)
    }
}

pub fn post(input: TokenStream) -> TokenStream {
    quote! {
        methods!([post], #input)
    }
}

pub fn put(input: TokenStream) -> TokenStream {
    quote! {
        methods!([put], #input)
    }
}

pub fn patch(input: TokenStream) -> TokenStream {
    quote! {
        methods!([patch], #input)
    }
}

pub fn delete(input: TokenStream) -> TokenStream {
    quote! {
        methods!([delete], #input)
    }
}
