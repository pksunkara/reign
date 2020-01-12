use super::{Expr, Tokenize, ViewFields};
use proc_macro2::{Span, TokenStream};
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
                _ => return Err(Error::new(Span::call_site(), "expected binary operation")),
            }
        }
    }
}

impl Tokenize for ExprBinary {
    fn tokenize(&self, tokens: &mut TokenStream, idents: &mut ViewFields, scopes: &ViewFields) {
        self.left.tokenize(tokens, idents, scopes);
        self.op.to_tokens(tokens);
        self.right.tokenize(tokens, idents, scopes);
    }
}
