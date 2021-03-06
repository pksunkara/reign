use super::{Error, Parse, ParseStream, Tokenize, ViewFields};
use proc_macro2::{Span, TokenStream};
use quote::{quote, TokenStreamExt};
use syn::LitStr;

#[derive(Debug)]
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
    fn tokenize(&self, tokens: &mut TokenStream, _: &mut ViewFields, _: &ViewFields) {
        let content = format!("<!--{}-->", self.content);
        let comment_str = LitStr::new(&content, Span::call_site());

        tokens.append_all(quote! {
            write!(f, "{}", #comment_str)?;
        });
    }
}
