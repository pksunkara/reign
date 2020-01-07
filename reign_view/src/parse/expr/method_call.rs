use super::Expr;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    token::{Comma, Dot, Paren},
    Error, Ident, MethodTurbofish,
};

pub struct ExprMethodCall {
    pub receiver: Box<Expr>,
    pub dot_token: Dot,
    pub method: Ident,
    pub turbofish: Option<MethodTurbofish>,
    pub paren_token: Paren,
    pub args: Punctuated<Expr, Comma>,
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
