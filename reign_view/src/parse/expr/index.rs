use super::{Expr, Tokenize, ViewFields};
use proc_macro2::{Span, TokenStream};
use syn::{
    parse::{Parse, ParseStream, Result},
    token::Bracket,
    Error,
};

pub struct ExprIndex {
    pub expr: Box<Expr>,
    pub bracket_token: Bracket,
    pub index: Box<Expr>,
}

impl Parse for ExprIndex {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut expr: Expr = input.parse()?;
        loop {
            match expr {
                Expr::Index(inner) => return Ok(inner),
                Expr::Group(next) => expr = *next.expr,
                _ => {
                    return Err(Error::new(
                        Span::call_site(),
                        "expected indexing expression",
                    ))
                }
            }
        }
    }
}

impl Tokenize for ExprIndex {
    fn tokenize(&self, tokens: &mut TokenStream, idents: &mut ViewFields, scopes: &ViewFields) {
        self.expr.tokenize(tokens, idents, scopes);

        self.bracket_token.surround(tokens, |tokens| {
            self.index.tokenize(tokens, idents, scopes);
        });
    }
}
