use super::{Expr, Tokenize, ViewFields};
use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream, Result},
    token::And,
    Error,
};

pub struct ExprReference {
    pub and_token: And,
    pub expr: Box<Expr>,
}

impl Parse for ExprReference {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut expr: Expr = input.parse()?;
        loop {
            match expr {
                Expr::Reference(inner) => return Ok(inner),
                Expr::Group(next) => expr = *next.expr,
                _ => {
                    return Err(Error::new(
                        Span::call_site(),
                        "expected referencing operation",
                    ))
                }
            }
        }
    }
}

impl Tokenize for ExprReference {
    fn tokenize(&self, tokens: &mut TokenStream, idents: &mut ViewFields, scopes: &ViewFields) {
        self.and_token.to_tokens(tokens);
        self.expr.tokenize(tokens, idents, scopes);
    }
}
