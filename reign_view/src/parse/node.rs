use super::{tag_name_regex, Comment, Element, Error, Parse, ParseStream, Text, Tokenize};
use proc_macro2::TokenStream;

#[derive(Debug, PartialEq)]
pub enum Node {
    Element(Element),
    Comment(Comment),
    Text(Text),
}

impl Parse for Node {
    fn parse(input: &mut ParseStream) -> Result<Self, Error> {
        if input.cursor == 0 {
            input.skip_spaces()?;
        }

        if input.peek("<!--") {
            Ok(Node::Comment(input.parse()?))
        } else if input.is_match(&tag_name_regex()) {
            Ok(Node::Element(input.parse()?))
        } else {
            let text: Text = input.parse()?;

            if text.content.is_empty() {
                return Err(input.error("unable to continue parsing"));
            }

            Ok(Node::Text(text))
        }
    }
}

impl Tokenize for Node {
    fn tokenize(&self) -> TokenStream {
        match self {
            Node::Element(e) => e.tokenize(),
            Node::Comment(c) => c.tokenize(),
            Node::Text(t) => t.tokenize(),
        }
    }
}
