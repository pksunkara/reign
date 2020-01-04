use super::{Error, Parse, ParseStream, Tokenize};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::LitStr;

#[derive(Debug, PartialEq)]
pub struct Comment {
    pub content: String,
}

impl Parse for Comment {
    fn parse(input: &mut ParseStream) -> Result<Self, Error> {
        input.step("<!--")?;

        Ok(Comment {
            content: input.until("-->", true)?,
        })
    }
}

impl Tokenize for Comment {
    fn tokenize(&self) -> TokenStream {
        let content = format!("<!--{}-->", self.content);
        let comment_str = LitStr::new(&content, Span::call_site());

        quote! {
            write!(f, #comment_str)?;
        }
    }
}
