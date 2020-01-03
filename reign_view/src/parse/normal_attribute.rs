use super::consts::*;
use super::{AttributeValue, Error, Parse, ParseStream, Tokenize};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::LitStr;

#[derive(Debug, PartialEq)]
pub struct NormalAttribute {
    pub name: String,
    pub value: AttributeValue,
}

impl Parse for NormalAttribute {
    fn parse(input: &mut ParseStream) -> Result<Self, Error> {
        Ok(NormalAttribute {
            name: input.matched(ATTR_NAME)?,
            value: input.parse()?,
        })
    }
}

impl Tokenize for NormalAttribute {
    fn tokenize(&self) -> TokenStream {
        if REIGN_ATTR_NAMES.contains(&&self.name.as_str()) {
            return quote! {};
        }

        // TODO: If name has `:` at the beginning, wrap value in `{{  }}`

        let name = LitStr::new(&format!(" {}=", &self.name), Span::call_site());
        let value = self.value.tokenize();

        quote! {
            write!(f, #name);
            #value
        }
    }
}
