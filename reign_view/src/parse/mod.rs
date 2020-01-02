mod error;
mod parse_stream;

pub use error::Error;
pub use parse_stream::ParseStream;

const VOID_ELEMENTS: [&str; 14] = [
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source",
    "track", "wbr",
];

const TAG_REGEX: &str = "<([[:alpha:]][a-zA-Z0-9\\-]*[[:alnum:]]|[[:alpha:]])";
const ATTR_NAME: &str = "[^\\s\"\'>/=]+";
const ATTR_SYMBOL: &str = ":";
const DY_ATTR_NAME_PART: &str = "[^\\[\\]\\s\"\'>/=]*";
const DY_ATTR_EXPR: &str = "\\[([^=]+)\\]";
const ATTR_VALUE_DOUBLE_QUOTED: &str = "\"([^\"]*)\"";
const ATTR_VALUE_SINGLE_QUOTED: &str = "'([^']*)'";
const ATTR_VALUE_UNQUOTED: &str = "[^\\s\"'=<>`]+";

pub trait Parse: Sized {
    fn parse(input: &mut ParseStream) -> Result<Self, Error>;
}

#[derive(Debug, PartialEq)]
pub struct Comment {
    pub content: String,
}

impl Parse for Comment {
    fn parse(input: &mut ParseStream) -> Result<Self, Error> {
        input.step("<!--")?;

        Ok(Comment {
            content: input.until("-->", true)?,
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct DynamicAttribute {
    pub symbol: String,
    pub prefix: String,
    pub expr: String,
    pub suffix: String,
    pub value: String,
}

impl Parse for DynamicAttribute {
    fn parse(input: &mut ParseStream) -> Result<Self, Error> {
        Ok(DynamicAttribute {
            symbol: input.step(":")?,
            prefix: input.matched(DY_ATTR_NAME_PART)?,
            expr: input.capture(DY_ATTR_EXPR, 1)?,
            suffix: input.matched(DY_ATTR_NAME_PART)?,
            value: {
                input.skip_spaces()?;

                if input.peek("=") {
                    input.step("=")?;
                    input.skip_spaces()?;

                    if input.peek("\"") {
                        input.capture(ATTR_VALUE_DOUBLE_QUOTED, 1)?
                    } else if input.peek("'") {
                        input.capture(ATTR_VALUE_SINGLE_QUOTED, 1)?
                    } else {
                        input.matched(ATTR_VALUE_UNQUOTED)?
                    }
                } else {
                    "".to_string()
                }
            },
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct NormalAttribute {
    pub name: String,
    pub value: String,
}

impl Parse for NormalAttribute {
    fn parse(input: &mut ParseStream) -> Result<Self, Error> {
        Ok(NormalAttribute {
            name: input.matched(ATTR_NAME)?,
            value: {
                input.skip_spaces()?;

                if input.peek("=") {
                    input.step("=")?;
                    input.skip_spaces()?;

                    if input.peek("\"") {
                        input.capture(ATTR_VALUE_DOUBLE_QUOTED, 1)?
                    } else if input.peek("'") {
                        input.capture(ATTR_VALUE_SINGLE_QUOTED, 1)?
                    } else {
                        input.matched(ATTR_VALUE_UNQUOTED)?
                    }
                } else {
                    "".to_string()
                }
            },
        })
    }
}

#[derive(Debug, PartialEq)]
pub enum Attribute {
    Normal(NormalAttribute),
    Dynamic(DynamicAttribute),
}

impl Parse for Attribute {
    fn parse(input: &mut ParseStream) -> Result<Self, Error> {
        let dy_attr_regex = format!(
            "{}{part}{}{part}",
            ATTR_SYMBOL,
            DY_ATTR_EXPR,
            part = DY_ATTR_NAME_PART
        );

        if input.is_match(dy_attr_regex.as_str()) {
            Ok(Attribute::Dynamic(input.parse()?))
        } else if input.is_match(ATTR_NAME) {
            Ok(Attribute::Normal(input.parse()?))
        } else {
            Err(input.error("unable to parse attribute"))
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Element {
    pub name: String,
    pub attrs: Vec<Attribute>,
    pub children: Vec<Node>,
}

impl Parse for Element {
    fn parse(input: &mut ParseStream) -> Result<Self, Error> {
        let name = input.capture(TAG_REGEX, 1)?;

        Ok(Element {
            name: name.clone(),
            attrs: {
                let mut attrs = vec![];
                input.skip_spaces()?;

                while !input.peek("/>") && !input.peek(">") {
                    attrs.push(input.parse()?);
                    input.skip_spaces()?;
                }

                attrs
            },
            children: {
                let mut children = vec![];

                if input.peek("/>") {
                    input.step("/>")?;
                } else {
                    // input.peek(">") is true here
                    input.step(">")?;

                    if !VOID_ELEMENTS.contains(&name.as_str()) {
                        let closing_tag = format!("</{}", name);

                        while !input.peek(&closing_tag) {
                            let child = input.parse()?;
                            children.push(child);
                        }

                        input.step(&closing_tag)?;
                        input.skip_spaces()?;
                        input.step(">")?;
                    }
                }

                children
            },
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct Text {
    pub content: String,
}

impl Parse for Text {
    fn parse(input: &mut ParseStream) -> Result<Self, Error> {
        Ok(Text {
            content: input.until("<", false)?,
        })
    }
}

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
        } else if input.is_match(TAG_REGEX) {
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

pub fn parse(data: String) -> Node {
    ParseStream::new(data).parse::<Node>().unwrap()
}
