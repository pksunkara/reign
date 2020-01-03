use super::{Error, Parse, ParseStream, Tokenize};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::LitStr;

#[derive(Debug, PartialEq)]
pub struct Text {
    pub content: String,
}

impl Parse for Text {
    fn parse(input: &mut ParseStream) -> Result<Self, Error> {
        Ok(Text {
            content: input.until("<", false)?,
        })
    }
}

impl Tokenize for Text {
    fn tokenize(&self) -> TokenStream {
        // TODO: {{ var }} in text
        let text_str = LitStr::new(&self.content, Span::call_site());

        quote! {
            write!(f, #text_str);
        }
    }
}
