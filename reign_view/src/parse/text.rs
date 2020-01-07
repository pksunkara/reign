use super::{parse_expr, Error, Parse, ParseStream, Tokenize};
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{Ident, LitStr};

#[derive(Debug, PartialEq)]
pub enum TextPart {
    Normal(String),
    Expr(String),
}

impl Tokenize for TextPart {
    fn tokenize(&self, tokens: &mut TokenStream, idents: &mut Vec<Ident>) {
        match self {
            TextPart::Normal(n) => {
                let lit = LitStr::new(&n, Span::call_site());
                lit.to_tokens(tokens);
            }
            TextPart::Expr(e) => {
                let expr = parse_expr(&e).unwrap();
                expr.tokenize(tokens, idents);
            }
        }
    }
}

#[derive(Debug)]
pub struct Text {
    pub content: Vec<TextPart>,
}

impl Parse for Text {
    fn parse(input: &mut ParseStream) -> Result<Self, Error> {
        Ok(Text {
            content: input.parse_text()?,
        })
    }
}

impl Tokenize for Text {
    fn tokenize(&self, tokens: &mut TokenStream, idents: &mut Vec<Ident>) {
        let format_arg_str = "{}".repeat(self.content.len());
        let format_arg_lit = LitStr::new(&format_arg_str, Span::call_site());

        let content: Vec<TokenStream> = self
            .content
            .iter()
            .map(|x| {
                let mut ts = TokenStream::new();

                x.tokenize(&mut ts, idents);
                ts
            })
            .collect();

        tokens.append_all(quote! {
            write!(f, #format_arg_lit, #(#content),*)?;
        });
    }
}
