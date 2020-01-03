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

pub use attribute::Attribute;
pub use attribute_value::AttributeValue;
pub use comment::Comment;
pub use dynamic_attribute::DynamicAttribute;
pub use element::Element;
pub use error::Error;
pub use node::Node;
pub use normal_attribute::NormalAttribute;
pub use parse_stream::ParseStream;
pub use text::Text;

fn tag_name_regex() -> String {
    format!("<({0}(:?:{0})*)", TAG_NAME)
}

fn dy_attr_regex() -> String {
    format!("{}{2}{}{2}", ATTR_SYMBOL, DY_ATTR_EXPR, DY_ATTR_NAME_PART)
}

pub trait Parse: Sized {
    fn parse(input: &mut ParseStream) -> Result<Self, Error>;
}

pub trait Tokenize {
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
