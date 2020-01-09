use super::super::consts::*;
use super::{Code, Error, Parse, ParseStream, Tokenize};
use proc_macro2::{Span, TokenStream};
use quote::{quote, TokenStreamExt};
use syn::{Ident, LitStr};

#[derive(Debug)]
pub struct VariableAttribute {
    pub name: String,
    pub value: Code,
}

impl Parse for VariableAttribute {
    fn parse(input: &mut ParseStream) -> Result<Self, Error> {
        Ok(VariableAttribute {
            name: input.matched(ATTR_NAME)?,
            value: Code::parse_expr(input)?,
        })
    }
}

impl Tokenize for VariableAttribute {
    fn tokenize(&self, tokens: &mut TokenStream, idents: &mut Vec<Ident>, scopes: &[Ident]) {
        let name = LitStr::new(&format!(" {}=", &self.name), Span::call_site());

        tokens.append_all(quote! {
            write!(f, "{}", #name)?;
        });

        // self.value.tokenize(tokens, idents, scopes);
    }
}
