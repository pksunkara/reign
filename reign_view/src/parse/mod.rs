mod error;
mod parse_stream;

pub use error::Error;
pub use parse_stream::ParseStream;

const VOID_ELEMENTS: [&str; 14] = [
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link", "meta", "param", "source",
    "track", "wbr",
];

const TAG_REGEX: &str = "<([[:alpha:]][a-zA-Z0-9\\-]*)";
const ATTRIBUTE_NAME_REGEX: &str = "[^\\s\"\'>/=]+";
const ATTRIBUTE_VALUE_DOUBLE_QUOTED_REGEX: &str = "\"([^\"]*)\"";
const ATTRIBUTE_VALUE_SINGLE_QUOTED_REGEX: &str = "'([^']*)'";
const ATTRIBUTE_VALUE_UNQUOTED_REGEX: &str = "[^\\s\"'=<>`]+";

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
pub struct Attribute {
    pub name: String,
    pub value: String,
}

impl Parse for Attribute {
    fn parse(input: &mut ParseStream) -> Result<Self, Error> {
        Ok(Attribute {
            name: input.matched(ATTRIBUTE_NAME_REGEX)?,
            value: {
                input.skip_spaces()?;

                if input.peek("=") {
                    input.step("=")?;
                    input.skip_spaces()?;

                    if input.peek("\"") {
                        input.capture(ATTRIBUTE_VALUE_DOUBLE_QUOTED_REGEX, 1)?
                    } else if input.peek("'") {
                        input.capture(ATTRIBUTE_VALUE_SINGLE_QUOTED_REGEX, 1)?
                    } else {
                        input.matched(ATTRIBUTE_VALUE_UNQUOTED_REGEX)?
                    }
                } else {
                    "".to_string()
                }
            },
        })
    }
}

#[derive(Debug, PartialEq)]
pub struct Tag {
    pub name: String,
    pub attrs: Vec<Attribute>,
    pub children: Vec<Element>,
}

impl Parse for Tag {
    fn parse(input: &mut ParseStream) -> Result<Self, Error> {
        let name = input.capture(TAG_REGEX, 1)?;
        println!("name: {}", name);

        Ok(Tag {
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
                        println!("closing: {}", closing_tag);

                        while !input.peek(&closing_tag) {
                            let child = input.parse()?;
                            println!("child: {:#?}", child);
                            println!("content: {}", input.content.get(input.cursor..).unwrap());
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
pub enum Element {
    Tag(Tag),
    Comment(Comment),
    Text(Text),
}

impl Parse for Element {
    fn parse(input: &mut ParseStream) -> Result<Self, Error> {
        if input.cursor == 0 {
            input.skip_spaces()?;
        }

        if input.peek("<!--") {
            Ok(Element::Comment(input.parse()?))
        } else if input.is_match(TAG_REGEX) {
            Ok(Element::Tag(input.parse()?))
        } else {
            let text: Text = input.parse()?;

            if text.content.is_empty() {
                return Err(input.error("unable to continue parsing"));
            }

            Ok(Element::Text(text))
        }
    }
}

pub fn parse(data: String) -> Element {
    ParseStream::new(data).parse::<Element>().unwrap()
}
