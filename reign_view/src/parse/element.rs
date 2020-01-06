use super::consts::*;
use super::{
    tag_name_regex, Attribute, Error, Node, NormalAttribute, Parse, ParseStream, Tokenize,
};
use inflector::cases::pascalcase::to_pascal_case;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Ident, LitStr};

#[derive(Debug)]
pub struct Element {
    pub name: String,
    pub attrs: Vec<Attribute>,
    pub children: Vec<Node>,
}

impl Element {
    fn attr(&self, name: &str) -> Option<&NormalAttribute> {
        for attr in &self.attrs {
            if let Attribute::Normal(normal) = attr {
                if normal.name == name {
                    return Some(normal);
                }
            }
        }

        None
    }

    fn slot_name(&self) -> &str {
        if let Some(attr) = self.attr("name") {
            return attr.value.value();
        }

        "default"
    }

    fn component_children(&self) -> (Vec<LitStr>, Vec<TokenStream>, Vec<TokenStream>) {
        let mut names = vec![];
        let mut templates = vec![];
        let mut other = vec![];

        for child in &self.children {
            let ts = child.tokenize();

            if let Node::Element(e) = child {
                if e.name == "template" {
                    for attr in &e.attrs {
                        if let Attribute::Normal(n) = attr {
                            if n.name.starts_with('#') {
                                names
                                    .push(LitStr::new(n.name.get(1..).unwrap(), Span::call_site()));
                                templates.push(ts);
                                break;
                            }
                        }
                    }
                } else {
                    other.push(ts);
                }
            } else {
                other.push(ts);
            }
        }

        (names, templates, other)
    }

    fn children_tokens(&self) -> Vec<TokenStream> {
        let mut tokens = vec![];
        let mut iter = self.children.iter();
        let mut child_option = iter.next();

        while child_option.is_some() {
            let child = child_option.unwrap();

            if let Node::Element(e) = child {
                if e.attr("!if").is_some() {
                    let mut after_if = vec![child];
                    let mut next = iter.next();
                    let (mut has_else, mut has_else_if) = (false, false);

                    while next.is_some() {
                        let sibling = next.unwrap();

                        if let Node::Element(e) = sibling {
                            // If element has `else`, Mark the children to be cleaned
                            if e.attr("!else").is_some() {
                                after_if.push(sibling);
                                has_else = true;
                                child_option = iter.next();
                                break;
                            }

                            // If element has `else-if`, mark the children to be cleaned even though we have no `else`
                            if e.attr("!else-if").is_some() {
                                has_else_if = true;
                            } else {
                                // Otherwise go to the main loop
                                child_option = next;
                                break;
                            }
                        }

                        after_if.push(sibling);
                        next = iter.next();
                    }

                    after_if = clean_if_else_group(after_if, has_else, has_else_if);

                    for i in after_if {
                        tokens.push(i.tokenize());
                    }

                    // If at the end, break out
                    if next.is_none() {
                        break;
                    }

                    continue;
                }

                if e.attr("!else").is_some() || e.attr("!else-if").is_some() {
                    // TODO:(view:err) Show the error position
                    // panic!("expected `!if` element before `!else` or `!else-if`");
                }
            }

            tokens.push(child.tokenize());
            child_option = iter.next();
        }

        tokens
    }
}

impl Parse for Element {
    fn parse(input: &mut ParseStream) -> Result<Self, Error> {
        let name = input.capture(&tag_name_regex(), 1)?;

        Ok(Element {
            name: name.to_lowercase(),
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

                    // TODO:(view:html) Tags that can be left open according to HTML spec
                    if !VOID_TAGS.contains(&name.as_str()) {
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

impl Tokenize for Element {
    fn tokenize(&self) -> TokenStream {
        let tag_pieces: Vec<&str> = self.name.split(':').collect();

        let tokens = if tag_pieces.len() == 1 && is_reserved_tag(&self.name) {
            let start_tag = LitStr::new(&format!("<{}", &self.name), Span::call_site());
            let attrs: Vec<TokenStream> = self.attrs.iter().map(|x| x.tokenize()).collect();
            let children = self.children_tokens();

            let end_tokens = if !VOID_TAGS.contains(&self.name.as_str()) {
                let end_tag = LitStr::new(&format!("</{}>", &self.name), Span::call_site());

                quote! {
                    write!(f, "{}", #end_tag)?;
                }
            } else {
                quote! {}
            };

            quote! {
                write!(f, "{}", #start_tag)?;
                #(#attrs)*
                write!(f, ">")?;
                #(#children)*
                #end_tokens
            }
        } else if self.name == "slot" {
            let name = LitStr::new(self.slot_name(), Span::call_site());

            quote! {
                self._slots.render(f, #name)?;
            }
        } else {
            let path = convert_tag_name(tag_pieces);
            let (names, templates, children) = self.component_children();

            // TODO:(attrs) For component
            quote! {
                write!(f, "{}", crate::views::#(#path)::* {
                    _slots: ::reign::view::Slots {
                        templates: ::maplit::hashmap!{
                            #(#names => Box::new(|f| {
                                #templates
                                Ok(())
                            })),*
                        },
                        children: Box::new(|f| {
                            #(#children)*
                            Ok(())
                        }),
                    },
                })?;
            }
        };

        // For loop
        if let Some(r_for) = self.attr("!for") {
            let for_expr = r_for.value.for_expr();

            return quote! {
                for #for_expr {
                    #tokens
                }
            };
        }

        // If condition
        if let Some(r_if) = self.attr("!if") {
            let if_expr = r_if.value.if_expr();

            return quote! {
                if #if_expr {
                    #tokens
                }
            };
        }

        // Else If condition
        if let Some(r_else_if) = self.attr("!else-if") {
            let if_expr = r_else_if.value.if_expr();

            return quote! {
                else if #if_expr {
                    #tokens
                }
            };
        }

        // Else condition
        if self.attr("!else").is_some() {
            return quote! {
                else {
                    #tokens
                }
            };
        }

        tokens
    }
}

fn clean_if_else_group(group: Vec<&Node>, has_else: bool, has_else_if: bool) -> Vec<&Node> {
    if has_else {
        // Clean completely
        group
            .into_iter()
            .filter(|x| {
                if let Node::Element(_) = x {
                    true
                } else {
                    false
                }
            })
            .collect()
    } else if has_else_if {
        // Clean only between if and else_if
        let mut last_element = group
            .iter()
            .rev()
            .position(|x| {
                if let Node::Element(_) = x {
                    true
                } else {
                    false
                }
            })
            .unwrap();

        last_element = group.len() - last_element - 1;

        group
            .into_iter()
            .enumerate()
            .filter(|(i, x)| {
                if *i > last_element {
                    return true;
                }

                if let Node::Element(_) = x {
                    true
                } else {
                    false
                }
            })
            .map(|(_, x)| x)
            .collect()
    } else {
        group
    }
}

fn convert_tag_name(tag: Vec<&str>) -> Vec<Ident> {
    let mut idents: Vec<Ident> = tag
        .into_iter()
        .map(|t| Ident::new(t, Span::call_site()))
        .collect();

    if let Some(ident) = idents.pop() {
        let new_ident = to_pascal_case(&ident.to_string());
        idents.push(Ident::new(&new_ident, Span::call_site()));
    }

    idents
}

fn is_reserved_tag(tag: &str) -> bool {
    SVG_TAGS.contains(&tag) || HTML_TAGS.contains(&tag)
}
