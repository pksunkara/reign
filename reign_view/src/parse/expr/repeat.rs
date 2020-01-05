use super::Expr;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream, Result},
    token::{Bracket, Semi},
    Error,
};

#[derive(Debug)]
pub struct ExprRepeat {
    bracket_token: Bracket,
    expr: Box<Expr>,
    semi_token: Semi,
    len: Box<Expr>,
}

impl Parse for ExprRepeat {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut expr: Expr = input.parse()?;
        loop {
            match expr {
                Expr::Repeat(inner) => return Ok(inner),
                Expr::Group(next) => expr = *next.expr,
                _ => {
                    return Err(Error::new_spanned(
                        expr,
                        "expected array literal constructed from one repeated element",
                    ))
                }
            }
        }
    }
}

impl ToTokens for ExprRepeat {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.bracket_token.surround(tokens, |tokens| {
            self.expr.to_tokens(tokens);
            self.semi_token.to_tokens(tokens);
            self.len.to_tokens(tokens);
        });
    }
}
