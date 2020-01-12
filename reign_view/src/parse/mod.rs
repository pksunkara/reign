use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{
    punctuated::{Pair, Punctuated},
    Ident,
};

mod attribute;
mod code;
mod comment;
mod consts;
mod doctype;
mod element;
mod error;
mod expr;
mod node;
mod parse_stream;
mod pat;
mod text;
mod view_fields;

use attribute::Attribute;
use code::Code;
use comment::Comment;
use doctype::Doctype;
use element::Element;
use error::Error;
use expr::Expr;
use node::Node;
use parse_stream::ParseStream;
use pat::For;
use text::{Text, TextPart};
use view_fields::ViewFields;

fn tag_name_regex() -> String {
    format!("<({0}(:?:{0})*)", consts::TAG_NAME)
}

trait Parse: Sized {
    fn parse(input: &mut ParseStream) -> Result<Self, Error>;
}

trait Tokenize {
    fn tokenize(&self, tokens: &mut TokenStream, idents: &mut ViewFields, scopes: &ViewFields);
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

impl<T, P> Tokenize for Punctuated<T, P>
where
    T: Tokenize,
    P: ToTokens,
{
    fn tokenize(&self, tokens: &mut TokenStream, idents: &mut ViewFields, scopes: &ViewFields) {
        let mut iter = self.pairs();

        loop {
            let item = iter.next();

            if item.is_none() {
                break;
            }

            match item.unwrap() {
                Pair::Punctuated(t, p) => {
                    t.tokenize(tokens, idents, scopes);
                    p.to_tokens(tokens);
                }
                Pair::End(t) => t.tokenize(tokens, idents, scopes),
            }
        }
    }
}

impl<T> Tokenize for Option<Box<T>>
where
    T: Tokenize,
{
    fn tokenize(&self, tokens: &mut TokenStream, idents: &mut ViewFields, scopes: &ViewFields) {
        if self.is_some() {
            self.as_ref().unwrap().tokenize(tokens, idents, scopes);
        }
    }
}

pub fn tokenize(node: Node) -> (TokenStream, Vec<Ident>, Vec<Option<TokenStream>>) {
    let mut tokens = TokenStream::new();
    let mut idents = ViewFields::new();
    let scopes = ViewFields::new();

    node.tokenize(&mut tokens, &mut idents, &scopes);

    (
        tokens,
        idents.fields.keys().cloned().collect(),
        idents.fields.values().cloned().collect(),
    )
}
