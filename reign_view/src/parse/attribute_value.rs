use super::consts::*;
use super::{parse_expr, parse_for, Error, Parse, ParseStream, Tokenize};
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

    pub fn for_expr(&self, tokens: &mut TokenStream, idents: &mut Vec<Ident>) {
        let for_ = parse_for(self.value()).unwrap();

        // TODO:(pat) in expr
        for_.tokenize(tokens, idents);
    }

    pub fn if_expr(&self, tokens: &mut TokenStream, idents: &mut Vec<Ident>) {
        parse_expr(self.value()).unwrap().tokenize(tokens, idents);
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
    fn tokenize(&self, tokens: &mut TokenStream, idents: &mut Vec<Ident>) {
        if let AttributeValue::NoValue = self {
            tokens.append_all(quote! {
                write!(f, "\"\"")?;
            });
            return;
        }

        let string = match self {
            AttributeValue::SingleQuoted(s) => format!("'{}'", s),
            AttributeValue::DoubleQuoted(d) => format!("\"{}\"", d),
            AttributeValue::Unquoted(u) => format!("\"{}\"", u),
            _ => unreachable!(),
        };

        // TODO:(expr-attr) {{ var }} in value
        let value = LitStr::new(&string, Span::call_site());

        tokens.append_all(quote! {
            write!(f, "{}", #value)?;
        });
    }
}
