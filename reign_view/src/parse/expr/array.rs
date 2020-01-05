use super::Expr;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    token::{Bracket, Comma},
    Error,
};

#[derive(Debug)]
pub struct ExprArray {
    bracket_token: Bracket,
    elems: Punctuated<Expr, Comma>,
}

impl Parse for ExprArray {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut expr: Expr = input.parse()?;
        loop {
            match expr {
                Expr::Array(inner) => return Ok(inner),
                Expr::Group(next) => expr = *next.expr,
                _ => {
                    return Err(Error::new_spanned(
                        expr,
                        "expected slice literal expression",
                    ))
                }
            }
        }
    }
}

impl ToTokens for ExprArray {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.bracket_token.surround(tokens, |tokens| {
            self.elems.to_tokens(tokens);
        })
    }
}
