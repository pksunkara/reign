use super::Expr;
use proc_macro2::TokenStream;
use quote::ToTokens;
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
                    return Err(Error::new_spanned(
                        expr,
                        "expected parenthesized expression",
                    ))
                }
            }
        }
    }
}

impl ToTokens for ExprParen {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.paren_token.surround(tokens, |tokens| {
            self.expr.to_tokens(tokens);
        });
    }
}
