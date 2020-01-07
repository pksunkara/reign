use super::Expr;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    token::{Comma, Paren},
    Error,
};

pub struct ExprCall {
    pub func: Box<Expr>,
    pub paren_token: Paren,
    pub args: Punctuated<Expr, Comma>,
}

impl Parse for ExprCall {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut expr: Expr = input.parse()?;
        loop {
            match expr {
                Expr::Call(inner) => return Ok(inner),
                Expr::Group(next) => expr = *next.expr,
                _ => {
                    return Err(Error::new_spanned(
                        expr,
                        "expected function call expression",
                    ))
                }
            }
        }
    }
}

impl ToTokens for ExprCall {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.func.to_tokens(tokens);
        self.paren_token.surround(tokens, |tokens| {
            self.args.to_tokens(tokens);
        })
    }
}
