use super::consts::*;
use super::{Error, Parse, ParseStream, Tokenize};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::LitStr;

#[derive(Debug, PartialEq)]
pub enum AttributeValue {
    SingleQuoted(String),
    DoubleQuoted(String),
    Unquoted(String),
    NoValue,
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
    fn tokenize(&self) -> TokenStream {
        if let AttributeValue::NoValue = self {
            return quote! {
                write!(f, "\"\"");
            };
        }

        let string = match self {
            AttributeValue::SingleQuoted(s) => format!("'{}'", s),
            AttributeValue::DoubleQuoted(d) => format!("\"{}\"", d),
            AttributeValue::Unquoted(u) => format!("\"{}\"", u),
            _ => unreachable!(),
        };

        // TODO: {{ var }} in value
        let value = LitStr::new(&string, Span::call_site());

        quote! {
            write!(f, #value);
        }
    }
}

impl AttributeValue {
    pub fn for_expr(&self) -> TokenStream {
        quote! {}
    }
}
