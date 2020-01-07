use super::Expr;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream, Result},
    BinOp, Error,
};

pub struct ExprBinary {
    pub left: Box<Expr>,
    pub op: BinOp,
    pub right: Box<Expr>,
}

impl Parse for ExprBinary {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut expr: Expr = input.parse()?;
        loop {
            match expr {
                Expr::Binary(inner) => return Ok(inner),
                Expr::Group(next) => expr = *next.expr,
                _ => return Err(Error::new_spanned(expr, "expected binary operation")),
            }
        }
    }
}

impl ToTokens for ExprBinary {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.left.to_tokens(tokens);
        self.op.to_tokens(tokens);
        self.right.to_tokens(tokens);
    }
}
