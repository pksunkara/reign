use super::consts::DOCTYPE;
use super::{Error, Parse, ParseStream, Tokenize, ViewFields};
use proc_macro2::{Span, TokenStream};
use quote::{quote, TokenStreamExt};
use syn::LitStr;

#[derive(Debug)]
pub struct Doctype {
    pub content: String,
}

impl Parse for Doctype {
    fn parse(input: &mut ParseStream) -> Result<Self, Error> {
        Ok(Doctype {
            content: input.matched(DOCTYPE)?,
        })
    }
}

impl Tokenize for Doctype {
    fn tokenize(&self, tokens: &mut TokenStream, _: &mut ViewFields, _: &ViewFields) {
        let doctype_str = LitStr::new(&self.content, Span::call_site());

        tokens.append_all(quote! {
            write!(f, "{}", #doctype_str)?;
        });
    }
}
