use super::super::consts::*;
use super::{Error, Parse, ParseStream, Tokenize};
use proc_macro2::{Span, TokenStream};
use quote::{quote, TokenStreamExt};
use syn::{Ident, LitStr};

#[derive(Debug)]
pub enum AttributeValue {
    SingleQuoted(String),
    DoubleQuoted(String),
    Unquoted(String),
    NoValue,
}

impl AttributeValue {
    pub fn value(&self) -> &str {
        match self {
            AttributeValue::SingleQuoted(s) => s,
            AttributeValue::DoubleQuoted(d) => d,
            AttributeValue::Unquoted(u) => u,
            _ => "",
        }
    }
}

impl AttributeValue {
    pub fn parse_to_str(input: &mut ParseStream) -> Result<String, Error> {
        input.skip_spaces()?;

        if input.peek("=") {
            input.step("=")?;
            input.skip_spaces()?;

            if input.peek("\"") {
                Ok(input.capture(ATTR_VALUE_DOUBLE_QUOTED, 1)?)
            } else if input.peek("\'") {
                Ok(input.capture(ATTR_VALUE_SINGLE_QUOTED, 1)?)
            } else {
                Ok(input.matched(ATTR_VALUE_UNQUOTED)?)
            }
        } else {
            Ok("\"\"".to_string())
        }
    }
}

impl Parse for AttributeValue {
    fn parse(input: &mut ParseStream) -> Result<Self, Error> {
        input.skip_spaces()?;

        if input.peek("=") {
            input.step("=")?;
            input.skip_spaces()?;

            if input.peek("\"") {
                Ok(AttributeValue::DoubleQuoted(
                    input.capture(ATTR_VALUE_DOUBLE_QUOTED, 1)?,
                ))
            } else if input.peek("\'") {
                Ok(AttributeValue::SingleQuoted(
                    input.capture(ATTR_VALUE_SINGLE_QUOTED, 1)?,
                ))
            } else {
                Ok(AttributeValue::Unquoted(
                    input.matched(ATTR_VALUE_UNQUOTED)?,
                ))
            }
        } else {
            Ok(AttributeValue::NoValue)
        }
    }
}

impl Tokenize for AttributeValue {
    fn tokenize(&self, tokens: &mut TokenStream, _: &mut Vec<Ident>, _: &[Ident]) {
        let string = match self {
            AttributeValue::SingleQuoted(s) => s,
            AttributeValue::DoubleQuoted(d) => d,
            AttributeValue::Unquoted(u) => u,
            AttributeValue::NoValue => "",
        };

        let value = LitStr::new(&string, Span::call_site());

        tokens.append_all(quote! { #value });
    }
}
