use super::super::{consts::*, StringPart};
use super::{Error, Parse, ParseStream, Tokenize, ViewFields};
use proc_macro2::{Span, TokenStream};
use quote::{quote, TokenStreamExt};
use syn::LitStr;

#[derive(Debug)]
pub struct AttributeValue {
    pub parts: Vec<StringPart>,
}

impl AttributeValue {
    pub fn value(&self) -> Option<String> {
        let mut strings: Vec<String> = vec![];

        for part in &self.parts {
            if let StringPart::Normal(s) = part {
                strings.push(s.clone());
            } else {
                return None;
            }
        }

        Some(strings.join(""))
    }

    pub fn has_expr(&self) -> bool {
        for part in &self.parts {
            if let StringPart::Expr(_) = part {
                return true;
            }
        }

        false
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
        Ok(AttributeValue {
            parts: {
                let value = AttributeValue::parse_to_str(input)?;
                StringPart::parse(input, &value, true)?
            },
        })
    }
}

impl Tokenize for AttributeValue {
    fn tokenize(&self, tokens: &mut TokenStream, idents: &mut ViewFields, scopes: &ViewFields) {
        if !self.has_expr() {
            let mut string = self.value().unwrap();

            if string == "\"\"" {
                string = "".to_string();
            }

            // TODO:(view:html-escape)
            let value = LitStr::new(&string, Span::call_site());

            tokens.append_all(quote! { #value });
        } else {
            let mut ts = TokenStream::new();
            self.parts.tokenize(&mut ts, idents, scopes);

            tokens.append_all(quote! {
                format!(#ts)
            })
        }
    }
}
