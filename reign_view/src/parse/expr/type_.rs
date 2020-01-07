use super::{Expr, Tokenize};
use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream, Result},
    token::Colon,
    Error, Ident, Type,
};

pub struct ExprType {
    pub expr: Box<Expr>,
    pub colon_token: Colon,
    pub ty: Box<Type>,
}

impl Parse for ExprType {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut expr: Expr = input.parse()?;
        loop {
            match expr {
                Expr::Type(inner) => return Ok(inner),
                Expr::Group(next) => expr = *next.expr,
                _ => {
                    return Err(Error::new(
                        Span::call_site(),
                        "expected type ascription expression",
                    ))
                }
            }
        }
    }
}

impl Tokenize for ExprType {
    fn tokenize(&self, tokens: &mut TokenStream, idents: &mut Vec<Ident>) {
        self.expr.tokenize(tokens, idents);
        self.colon_token.to_tokens(tokens);
        self.ty.to_tokens(tokens);
    }
}
