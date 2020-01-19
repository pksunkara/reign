use super::{Expr, Tokenize, ViewFields};
use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream, Result},
    token::As,
    Error, Type,
};

pub struct ExprCast {
    pub expr: Box<Expr>,
    pub as_token: As,
    pub ty: Box<Type>,
}

impl Parse for ExprCast {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut expr: Expr = input.parse()?;
        loop {
            match expr {
                Expr::Cast(inner) => return Ok(inner),
                Expr::Group(next) => expr = *next.expr,
                _ => return Err(Error::new(Span::call_site(), "expected cast expression")),
            }
        }
    }
}

impl Tokenize for ExprCast {
    fn tokenize(&self, tokens: &mut TokenStream, idents: &mut ViewFields, scopes: &ViewFields) {
        // Can't use casting for type identification because it tries to copy the variable

        self.expr.tokenize(tokens, idents, scopes);
        self.as_token.to_tokens(tokens);
        self.ty.to_tokens(tokens);
    }
}
