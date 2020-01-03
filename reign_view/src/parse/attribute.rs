use super::consts::ATTR_NAME;
use super::{
    dy_attr_regex, DynamicAttribute, Error, NormalAttribute, Parse, ParseStream, Tokenize,
};
use proc_macro2::TokenStream;

#[derive(Debug, PartialEq)]
pub enum Attribute {
    Normal(NormalAttribute),
    Dynamic(DynamicAttribute),
}

impl Parse for Attribute {
    fn parse(input: &mut ParseStream) -> Result<Self, Error> {
        if input.is_match(&dy_attr_regex()) {
            Ok(Attribute::Dynamic(input.parse()?))
        } else if input.is_match(ATTR_NAME) {
            Ok(Attribute::Normal(input.parse()?))
        } else {
            Err(input.error("unable to parse attribute"))
        }
    }
}

impl Tokenize for Attribute {
    fn tokenize(&self) -> TokenStream {
        match self {
            Attribute::Normal(n) => n.tokenize(),
            Attribute::Dynamic(d) => d.tokenize(),
        }
    }
}
