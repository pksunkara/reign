use super::super::consts::*;
use super::{Code, Error, Parse, ParseStream};

#[derive(Debug)]
pub struct ControlAttribute {
    pub name: String,
    pub value: Code,
}

impl Parse for ControlAttribute {
    fn parse(input: &mut ParseStream) -> Result<Self, Error> {
        let name = input.capture(CTRL_ATTR, 1)?;

        Ok(ControlAttribute {
            name: name.clone(),
            value: {
                if name == "for" {
                    Code::parse_for(input)?
                } else {
                    Code::parse_expr(input)?
                }
            },
        })
    }
}
