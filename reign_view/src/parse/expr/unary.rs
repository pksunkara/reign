use super::Expr;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream, Result},
    Error, UnOp,
};

#[derive(Debug)]
pub struct ExprUnary {
    op: UnOp,
    expr: Box<Expr>,
}

impl Parse for ExprUnary {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut expr: Expr = input.parse()?;
        loop {
            match expr {
                Expr::Unary(inner) => return Ok(inner),
                Expr::Group(next) => expr = *next.expr,
                _ => return Err(Error::new_spanned(expr, "expected unary operation")),
            }
        }
    }
}

impl ToTokens for ExprUnary {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.op.to_tokens(tokens);
        self.expr.to_tokens(tokens);
    }
}
