use super::Expr;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream, Result},
    token::Colon,
    Error, Type,
};

#[derive(Debug)]
pub struct ExprType {
    expr: Box<Expr>,
    colon_token: Colon,
    ty: Box<Type>,
}

impl Parse for ExprType {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut expr: Expr = input.parse()?;
        loop {
            match expr {
                Expr::Type(inner) => return Ok(inner),
                Expr::Group(next) => expr = *next.expr,
                _ => {
                    return Err(Error::new_spanned(
                        expr,
                        "expected type ascription expression",
                    ))
                }
            }
        }
    }
}

impl ToTokens for ExprType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.expr.to_tokens(tokens);
        self.colon_token.to_tokens(tokens);
        self.ty.to_tokens(tokens);
    }
}
