use super::consts::*;
use super::{Code, Error, Parse, ParseStream, Tokenize, ViewFields};
use proc_macro2::TokenStream;

mod control;
mod dynamic;
mod normal;
mod value;
mod variable;

pub use control::ControlAttribute;
use dynamic::DynamicAttribute;
pub use normal::NormalAttribute;
pub use value::AttributeValue;
use variable::VariableAttribute;

#[derive(Debug)]
pub enum Attribute {
    Normal(NormalAttribute),
    Dynamic(DynamicAttribute),
    Variable(VariableAttribute),
    Control(ControlAttribute),
}

impl Parse for Attribute {
    fn parse(input: &mut ParseStream) -> Result<Self, Error> {
        if input.is_match(&dy_attr_regex()) {
            Ok(Attribute::Dynamic(input.parse()?))
        } else if input.is_match(&var_attr_regex()) {
            Ok(Attribute::Variable(input.parse()?))
        } else if input.is_match(CTRL_ATTR) {
            Ok(Attribute::Control(input.parse()?))
        } else if input.is_match(ATTR_NAME) {
            Ok(Attribute::Normal(input.parse()?))
        } else {
            Err(input.error("unable to parse attribute"))
        }
    }
}

impl Tokenize for Attribute {
    fn tokenize(&self, tokens: &mut TokenStream, idents: &mut ViewFields, scopes: &ViewFields) {
        match self {
            Attribute::Normal(n) => n.tokenize(tokens, idents, scopes),
            Attribute::Dynamic(d) => d.tokenize(tokens, idents, scopes),
            Attribute::Variable(v) => v.tokenize(tokens, idents, scopes),
            _ => {}
        };
    }
}

fn dy_attr_regex() -> String {
    format!(
        "{}{2}{}{2}",
        VAR_ATTR_SYMBOL, DY_ATTR_EXPR, DY_ATTR_NAME_PART
    )
}

fn var_attr_regex() -> String {
    format!("{}({})", VAR_ATTR_SYMBOL, ATTR_NAME)
}
