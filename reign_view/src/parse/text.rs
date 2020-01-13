use super::{Error, Parse, ParseStream, StringPart, Tokenize, ViewFields};
use proc_macro2::TokenStream;
use quote::{quote, TokenStreamExt};

#[derive(Debug)]
pub struct Text {
    pub content: Vec<StringPart>,
}

impl Parse for Text {
    fn parse(input: &mut ParseStream) -> Result<Self, Error> {
        Ok(Text {
            content: input.parse_text()?,
        })
    }
}

impl Tokenize for Text {
    fn tokenize(&self, tokens: &mut TokenStream, idents: &mut ViewFields, scopes: &ViewFields) {
        let mut ts = TokenStream::new();
        self.content.tokenize(&mut ts, idents, scopes);

        tokens.append_all(quote! {
            write!(f, #ts)?;
        })
    }
}
