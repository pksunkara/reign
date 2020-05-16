use super::{Expr, Tokenize, ViewFields};
use proc_macro2::{Span, TokenStream};
use syn::{
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    token::{Bracket, Comma},
    Error,
};

pub struct ExprArray {
    pub bracket_token: Bracket,
    pub elems: Punctuated<Expr, Comma>,
}

impl Parse for ExprArray {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut expr: Expr = input.parse()?;
        loop {
            match expr {
                Expr::Array(inner) => return Ok(inner),
                Expr::Group(next) => expr = *next.expr,
                _ => {
                    return Err(Error::new(
                        Span::call_site(),
                        "expected slice literal expression",
                    ))
                }
            }
        }
    }
}

impl Tokenize for ExprArray {
    fn tokenize(&self, tokens: &mut TokenStream, idents: &mut ViewFields, scopes: &ViewFields) {
        self.bracket_token.surround(tokens, |tokens| {
            self.elems.tokenize(tokens, idents, scopes);
        })
    }
}
