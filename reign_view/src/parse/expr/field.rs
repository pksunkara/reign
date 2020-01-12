use super::{Expr, Tokenize, ViewFields};
use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream, Result},
    token::Dot,
    Error, Member,
};

pub struct ExprField {
    pub base: Box<Expr>,
    pub dot_token: Dot,
    pub member: Member,
}

impl Parse for ExprField {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut expr: Expr = input.parse()?;
        loop {
            match expr {
                Expr::Field(inner) => return Ok(inner),
                Expr::Group(next) => expr = *next.expr,
                _ => {
                    return Err(Error::new(
                        Span::call_site(),
                        "expected struct field access",
                    ))
                }
            }
        }
    }
}

impl Tokenize for ExprField {
    fn tokenize(&self, tokens: &mut TokenStream, idents: &mut ViewFields, scopes: &ViewFields) {
        self.base.tokenize(tokens, idents, scopes);
        self.dot_token.to_tokens(tokens);
        self.member.to_tokens(tokens);
    }
}
