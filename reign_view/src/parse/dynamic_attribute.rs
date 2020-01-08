use super::consts::*;
use super::{AttributeValue, Error, Parse, ParseStream, Tokenize};
use proc_macro2::TokenStream;
use syn::Ident;

#[derive(Debug)]
pub struct DynamicAttribute {
    pub symbol: String,
    pub prefix: String,
    pub expr: String,
    pub suffix: String,
    pub value: AttributeValue,
}

impl Parse for DynamicAttribute {
    fn parse(input: &mut ParseStream) -> Result<Self, Error> {
        Ok(DynamicAttribute {
            symbol: input.step(":")?,
            prefix: input.matched(DY_ATTR_NAME_PART)?,
            expr: input.capture(DY_ATTR_EXPR, 1)?,
            suffix: input.matched(DY_ATTR_NAME_PART)?,
            value: input.parse()?,
        })
    }
}

impl Tokenize for DynamicAttribute {
    fn tokenize(&self, tokens: &mut TokenStream, idents: &mut Vec<Ident>, scopes: &Vec<Ident>) {}
}
