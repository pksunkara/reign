use super::Expr;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    token::{Comma, Paren},
    Error,
};

pub struct ExprTuple {
    pub paren_token: Paren,
    pub elems: Punctuated<Expr, Comma>,
}

impl Parse for ExprTuple {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut expr: Expr = input.parse()?;
        loop {
            match expr {
                Expr::Tuple(inner) => return Ok(inner),
                Expr::Group(next) => expr = *next.expr,
                _ => return Err(Error::new_spanned(expr, "expected tuple expression")),
            }
        }
    }
}

impl ToTokens for ExprTuple {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.paren_token.surround(tokens, |tokens| {
            self.elems.to_tokens(tokens);

            // If we only have one argument, we need a trailing comma to
            // distinguish ExprTuple from ExprParen.
            if self.elems.len() == 1 && !self.elems.trailing_punct() {
                Comma::default().to_tokens(tokens);
            }
        })
    }
}
