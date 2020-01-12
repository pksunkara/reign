use super::{Expr, Tokenize, ViewFields};
use proc_macro2::{Span, TokenStream};
use syn::{
    parse::{Parse, ParseStream, Result},
    token::Paren,
    Error,
};

pub struct ExprParen {
    pub paren_token: Paren,
    pub expr: Box<Expr>,
}

impl Parse for ExprParen {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut expr: Expr = input.parse()?;
        loop {
            match expr {
                Expr::Paren(inner) => return Ok(inner),
                Expr::Group(next) => expr = *next.expr,
                _ => {
                    return Err(Error::new(
                        Span::call_site(),
                        "expected parenthesized expression",
                    ))
                }
            }
        }
    }
}

impl Tokenize for ExprParen {
    fn tokenize(&self, tokens: &mut TokenStream, idents: &mut ViewFields, scopes: &ViewFields) {
        self.paren_token.surround(tokens, |tokens| {
            self.expr.tokenize(tokens, idents, scopes);
        });
    }
}
