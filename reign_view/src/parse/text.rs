use super::{parse_expr, Error, Parse, ParseStream, Tokenize};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::LitStr;

#[derive(Debug, PartialEq)]
pub enum TextPart {
    Normal(String),
    Expr(String),
}

impl Tokenize for TextPart {
    fn tokenize(&self) -> TokenStream {
        match self {
            TextPart::Normal(n) => {
                let lit = LitStr::new(&n, Span::call_site());
                quote! {
                        #lit
                }
            }
            TextPart::Expr(e) => {
                let expr = parse_expr(&e).unwrap();

                quote! {
                    #expr
                }
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
    fn tokenize(&self) -> TokenStream {
        let format_arg_str = "{}".repeat(self.content.len());
        let format_arg_lit = LitStr::new(&format_arg_str, Span::call_site());
        let tokens: Vec<TokenStream> = self.content.iter().map(|x| x.tokenize()).collect();

        quote! {
            write!(f, #format_arg_lit, #(#tokens),*)?;
        }
    }
}
