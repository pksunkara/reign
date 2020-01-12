use super::{Expr, Tokenize, ViewFields};
use proc_macro2::{Span, TokenStream};
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
                _ => {
                    return Err(Error::new(
                        Span::call_site(),
                        "expected method call expression",
                    ))
                }
            }
        }
    }
}

impl Tokenize for ExprMethodCall {
    fn tokenize(&self, tokens: &mut TokenStream, idents: &mut ViewFields, scopes: &ViewFields) {
        self.receiver.tokenize(tokens, idents, scopes);
        self.dot_token.to_tokens(tokens);
        self.method.to_tokens(tokens);
        self.turbofish.to_tokens(tokens);

        self.paren_token.surround(tokens, |tokens| {
            self.args.tokenize(tokens, idents, scopes);
        });
    }
}
