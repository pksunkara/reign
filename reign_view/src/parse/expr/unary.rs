use super::{Expr, Tokenize, ViewFields};
use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream, Result},
    Error, UnOp,
};

pub struct ExprUnary {
    pub op: UnOp,
    pub expr: Box<Expr>,
}

impl Parse for ExprUnary {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut expr: Expr = input.parse()?;
        loop {
            match expr {
                Expr::Unary(inner) => return Ok(inner),
                Expr::Group(next) => expr = *next.expr,
                _ => return Err(Error::new(Span::call_site(), "expected unary operation")),
            }
        }
    }
}

impl Tokenize for ExprUnary {
    fn tokenize(&self, tokens: &mut TokenStream, idents: &mut ViewFields, scopes: &ViewFields) {
        self.op.to_tokens(tokens);
        self.expr.tokenize(tokens, idents, scopes);
    }
}
