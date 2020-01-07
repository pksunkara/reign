mod attribute;
mod attribute_value;
mod comment;
mod consts;
mod doctype;
mod dynamic_attribute;
mod element;
mod error;
mod expr;
mod for_;
mod node;
mod normal_attribute;
mod parse_stream;
mod text;

use consts::*;
use proc_macro2::TokenStream;
use syn::{parse_str, Ident};

use attribute::Attribute;
use attribute_value::AttributeValue;
use comment::Comment;
use doctype::Doctype;
use dynamic_attribute::DynamicAttribute;
use element::Element;
use error::Error;
use expr::Expr;
use for_::For;
use node::Node;
use normal_attribute::NormalAttribute;
use parse_stream::ParseStream;
use text::{Text, TextPart};

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
    fn tokenize(&self, tokens: &mut TokenStream, idents: &mut Vec<Ident>);
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

pub fn parse_expr(data: &str) -> Result<Expr, Error> {
    let parsed = parse_str::<Expr>(data);

    if let Err(err) = parsed {
        // TODO:(view:err) Show the error like others
        panic!(err)
    } else {
        Ok(parsed.unwrap())
    }
}

pub fn parse_for(data: &str) -> Result<For, Error> {
    let parsed = parse_str::<For>(data);

    // TODO:(view:err) Show the error like others
    if let Err(err) = parsed {
        panic!(err)
    } else {
        Ok(parsed.unwrap())
    }
}

pub fn tokenize(node: Node) -> (TokenStream, Vec<Ident>) {
    let mut tokens = TokenStream::new();
    let mut idents = vec![];

    node.tokenize(&mut tokens, &mut idents);

    (tokens, idents)
}
