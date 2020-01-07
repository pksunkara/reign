use super::Expr;
use proc_macro2::TokenStream;
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
                _ => return Err(Error::new_spanned(expr, "expected cast expression")),
            }
        }
    }
}

impl ToTokens for ExprCast {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.expr.to_tokens(tokens);
        self.as_token.to_tokens(tokens);
        self.ty.to_tokens(tokens);
    }
}
