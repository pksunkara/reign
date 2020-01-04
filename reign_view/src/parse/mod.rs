mod attribute;
mod attribute_value;
mod comment;
mod consts;
mod dynamic_attribute;
mod element;
mod error;
mod node;
mod normal_attribute;
mod parse_stream;
mod text;

use consts::*;
use proc_macro2::TokenStream;

use attribute::Attribute;
use attribute_value::AttributeValue;
use comment::Comment;
use dynamic_attribute::DynamicAttribute;
use element::Element;
use error::Error;
use node::Node;
use normal_attribute::NormalAttribute;
use parse_stream::ParseStream;
use text::Text;

fn tag_name_regex() -> String {
    format!("<({0}(:?:{0})*)", TAG_NAME)
}

fn dy_attr_regex() -> String {
    format!("{}{2}{}{2}", ATTR_SYMBOL, DY_ATTR_EXPR, DY_ATTR_NAME_PART)
}

trait Parse: Sized {
    fn parse(input: &mut ParseStream) -> Result<Self, Error>;
}

trait Tokenize {
    fn tokenize(&self) -> TokenStream;
}

pub fn parse(data: String) -> Result<Node, Error> {
    let mut ps = ParseStream::new(data);
    let node: Node = ps.parse()?;

    ps.skip_spaces()?;

    if ps.content.len() != ps.cursor {
        Err(ps.error("only one top-level node is allowed"))
    } else {
        Ok(node)
    }
}

pub fn tokenize(node: Node) -> TokenStream {
    node.tokenize()
}
