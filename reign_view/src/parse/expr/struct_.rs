use super::{Expr, FieldValue, Tokenize, ViewFields};
use proc_macro2::{Span, TokenStream};
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    token::{Brace, Comma, Dot2},
    Error, Path,
};

pub struct ExprStruct {
    pub path: Path,
    pub brace_token: Brace,
    pub fields: Punctuated<FieldValue, Comma>,
    pub dot2_token: Option<Dot2>,
    pub rest: Option<Box<Expr>>,
}

impl Parse for ExprStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut expr: Expr = input.parse()?;
        loop {
            match expr {
                Expr::Struct(inner) => return Ok(inner),
                Expr::Group(next) => expr = *next.expr,
                _ => {
                    return Err(Error::new(
                        Span::call_site(),
                        "expected struct literal expression",
                    ))
                }
            }
        }
    }
}

impl Tokenize for ExprStruct {
    fn tokenize(&self, tokens: &mut TokenStream, idents: &mut ViewFields, scopes: &ViewFields) {
        self.path.to_tokens(tokens);

        self.brace_token.surround(tokens, |tokens| {
            self.fields.tokenize(tokens, idents, scopes);

            if self.rest.is_some() {
                match self.dot2_token {
                    Some(t) => t.to_tokens(tokens),
                    None => Dot2::default().to_tokens(tokens),
                }

                self.rest.tokenize(tokens, idents, scopes);
            }
        })
    }
}
