use super::{attribute::AttributeValue, Error, Expr, For, ParseStream, Tokenize, ViewFields};
use proc_macro2::TokenStream;
use std::fmt::{Debug, Error as FError, Formatter};
use syn::parse_str;

pub enum Code {
    For(For),
    Expr(Expr),
}

impl Code {
    pub fn parse_for(input: &mut ParseStream) -> Result<Self, Error> {
        let string = AttributeValue::parse_to_str(input)?;
        Self::parse_for_from_str(input, &string)
    }

    pub fn parse_expr(input: &mut ParseStream) -> Result<Self, Error> {
        let string = AttributeValue::parse_to_str(input)?;
        Self::parse_expr_from_str(input, &string)
    }

    pub fn parse_for_from_str(input: &mut ParseStream, text: &str) -> Result<Self, Error> {
        let parsed = parse_str::<For>(text);

        if let Ok(code) = parsed {
            Ok(Code::For(code))
        } else {
            Err(input.error("expected pattern in expression"))
        }
    }

    pub fn parse_expr_from_str(input: &ParseStream, text: &str) -> Result<Self, Error> {
        let parsed = parse_str::<Expr>(text);

        if let Ok(code) = parsed {
            Ok(Code::Expr(code))
        } else {
            Err(input.error("expected expression"))
        }
    }
}

impl Tokenize for Code {
    fn tokenize(&self, tokens: &mut TokenStream, idents: &mut ViewFields, scopes: &ViewFields) {
        match self {
            Code::For(f) => f.tokenize(tokens, idents, scopes),
            Code::Expr(e) => e.tokenize(tokens, idents, scopes),
        }
    }
}

impl Debug for Code {
    fn fmt(&self, _: &mut Formatter) -> Result<(), FError> {
        Ok(())
    }
}
