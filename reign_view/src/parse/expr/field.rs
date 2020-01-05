use super::Expr;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream, Result},
    token::Dot,
    Error, Member,
};

#[derive(Debug)]
pub struct ExprField {
    base: Box<Expr>,
    dot_token: Dot,
    member: Member,
}

impl Parse for ExprField {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut expr: Expr = input.parse()?;
        loop {
            match expr {
                Expr::Field(inner) => return Ok(inner),
                Expr::Group(next) => expr = *next.expr,
                _ => return Err(Error::new_spanned(expr, "expected struct field access")),
            }
        }
    }
}

impl ToTokens for ExprField {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.base.to_tokens(tokens);
        self.dot_token.to_tokens(tokens);
        self.member.to_tokens(tokens);
    }
}
