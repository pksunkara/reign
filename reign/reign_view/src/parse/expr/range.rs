use super::{Expr, Tokenize, ViewFields};
use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream, Result},
    Error, RangeLimits,
};

pub struct ExprRange {
    pub from: Option<Box<Expr>>,
    pub limits: RangeLimits,
    pub to: Option<Box<Expr>>,
}

impl Parse for ExprRange {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut expr: Expr = input.parse()?;
        loop {
            match expr {
                Expr::Range(inner) => return Ok(inner),
                Expr::Group(next) => expr = *next.expr,
                _ => return Err(Error::new(Span::call_site(), "expected range expression")),
            }
        }
    }
}

impl Tokenize for ExprRange {
    fn tokenize(&self, tokens: &mut TokenStream, idents: &mut ViewFields, scopes: &ViewFields) {
        self.from.tokenize(tokens, idents, scopes);

        match &self.limits {
            RangeLimits::HalfOpen(t) => t.to_tokens(tokens),
            RangeLimits::Closed(t) => t.to_tokens(tokens),
        }

        self.to.tokenize(tokens, idents, scopes);
    }
}
