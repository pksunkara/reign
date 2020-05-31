use super::{Expr, Tokenize, ViewFields};
use proc_macro2::{Span, TokenStream};
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
                    return Err(Error::new(
                        Span::call_site(),
                        "expected function call expression",
                    ))
                }
            }
        }
    }
}

impl Tokenize for ExprCall {
    fn tokenize(&self, tokens: &mut TokenStream, idents: &mut ViewFields, scopes: &ViewFields) {
        self.func.tokenize(tokens, idents, scopes);
        self.paren_token.surround(tokens, |tokens| {
            self.args.tokenize(tokens, idents, scopes);
        })
    }
}
