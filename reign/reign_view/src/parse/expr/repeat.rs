use super::{Expr, Tokenize, ViewFields};
use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream, Result},
    token::{Bracket, Semi},
    Error,
};

pub struct ExprRepeat {
    pub bracket_token: Bracket,
    pub expr: Box<Expr>,
    pub semi_token: Semi,
    pub len: Box<Expr>,
}

impl Parse for ExprRepeat {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut expr: Expr = input.parse()?;
        loop {
            match expr {
                Expr::Repeat(inner) => return Ok(inner),
                Expr::Group(next) => expr = *next.expr,
                _ => {
                    return Err(Error::new(
                        Span::call_site(),
                        "expected array literal constructed from one repeated element",
                    ))
                }
            }
        }
    }
}

impl Tokenize for ExprRepeat {
    fn tokenize(&self, tokens: &mut TokenStream, idents: &mut ViewFields, scopes: &ViewFields) {
        self.bracket_token.surround(tokens, |tokens| {
            self.expr.tokenize(tokens, idents, scopes);
            self.semi_token.to_tokens(tokens);
            self.len.tokenize(tokens, idents, scopes);
        });
    }
}
