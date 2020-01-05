use super::Expr;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    token::{Comma, Dot, Paren},
    Error, Ident, MethodTurbofish,
};

#[derive(Debug)]
pub struct ExprMethodCall {
    receiver: Box<Expr>,
    dot_token: Dot,
    method: Ident,
    turbofish: Option<MethodTurbofish>,
    paren_token: Paren,
    args: Punctuated<Expr, Comma>,
}

impl Parse for ExprMethodCall {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut expr: Expr = input.parse()?;
        loop {
            match expr {
                Expr::MethodCall(inner) => return Ok(inner),
                Expr::Group(next) => expr = *next.expr,
                _ => return Err(Error::new_spanned(expr, "expected method call expression")),
            }
        }
    }
}

impl ToTokens for ExprMethodCall {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.receiver.to_tokens(tokens);
        self.dot_token.to_tokens(tokens);
        self.method.to_tokens(tokens);
        self.turbofish.to_tokens(tokens);

        self.paren_token.surround(tokens, |tokens| {
            self.args.to_tokens(tokens);
        });
    }
}
