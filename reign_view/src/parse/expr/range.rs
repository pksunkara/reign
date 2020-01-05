use super::Expr;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream, Result},
    Error, RangeLimits,
};

#[derive(Debug)]
pub struct ExprRange {
    from: Option<Box<Expr>>,
    limits: RangeLimits,
    to: Option<Box<Expr>>,
}

impl Parse for ExprRange {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut expr: Expr = input.parse()?;
        loop {
            match expr {
                Expr::Range(inner) => return Ok(inner),
                Expr::Group(next) => expr = *next.expr,
                _ => return Err(Error::new_spanned(expr, "expected range expression")),
            }
        }
    }
}

impl ToTokens for ExprRange {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.from.to_tokens(tokens);

        match &self.limits {
            RangeLimits::HalfOpen(t) => t.to_tokens(tokens),
            RangeLimits::Closed(t) => t.to_tokens(tokens),
        }

        self.to.to_tokens(tokens);
    }
}
