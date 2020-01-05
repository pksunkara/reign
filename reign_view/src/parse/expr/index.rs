use super::Expr;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream, Result},
    token::Bracket,
    Error,
};

#[derive(Debug)]
pub struct ExprIndex {
    expr: Box<Expr>,
    bracket_token: Bracket,
    index: Box<Expr>,
}

impl Parse for ExprIndex {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut expr: Expr = input.parse()?;
        loop {
            match expr {
                Expr::Index(inner) => return Ok(inner),
                Expr::Group(next) => expr = *next.expr,
                _ => return Err(Error::new_spanned(expr, "expected indexing expression")),
            }
        }
    }
}

impl ToTokens for ExprIndex {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.expr.to_tokens(tokens);

        self.bracket_token.surround(tokens, |tokens| {
            self.index.to_tokens(tokens);
        });
    }
}
