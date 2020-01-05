use super::{Expr, FieldValue};
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    token::{Brace, Comma, Dot2},
    Error, Path,
};

// TODO: FieldValue
#[derive(Debug)]
pub struct ExprStruct {
    path: Path,
    brace_token: Brace,
    fields: Punctuated<FieldValue, Comma>,
    dot2_token: Option<Dot2>,
    rest: Option<Box<Expr>>,
}

impl Parse for ExprStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut expr: Expr = input.parse()?;
        loop {
            match expr {
                Expr::Struct(inner) => return Ok(inner),
                Expr::Group(next) => expr = *next.expr,
                _ => {
                    return Err(Error::new_spanned(
                        expr,
                        "expected struct literal expression",
                    ))
                }
            }
        }
    }
}

impl ToTokens for ExprStruct {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.path.to_tokens(tokens);

        self.brace_token.surround(tokens, |tokens| {
            self.fields.to_tokens(tokens);

            if self.rest.is_some() {
                match self.dot2_token {
                    Some(t) => t.to_tokens(tokens),
                    None => Dot2::default().to_tokens(tokens),
                }

                self.rest.to_tokens(tokens);
            }
        })
    }
}
