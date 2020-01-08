use super::consts::*;
use super::{AttributeValue, Error, Parse, ParseStream, Tokenize};
use proc_macro2::{Span, TokenStream};
use quote::{quote, TokenStreamExt};
use syn::{Ident, LitStr};

#[derive(Debug)]
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
    fn tokenize(&self, tokens: &mut TokenStream, idents: &mut Vec<Ident>, scopes: &[Ident]) {
        if REIGN_ATTR_NAMES.contains(&&self.name.as_str()) {
            return;
        }

        // TODO:(expr-attr) If name has `:` at the beginning, wrap value in `{{  }}`

        let name = LitStr::new(&format!(" {}=", &self.name), Span::call_site());

        tokens.append_all(quote! {
            write!(f, "{}", #name)?;
        });
        self.value.tokenize(tokens, idents, scopes);
    }
}
