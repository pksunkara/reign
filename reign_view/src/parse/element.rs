use super::consts::*;
use super::{
    attribute::{ControlAttribute, NormalAttribute},
    tag_name_regex, Attribute, Code, Error, Node, Parse, ParseStream, Tokenize,
};
use inflector::cases::pascalcase::to_pascal_case;
use proc_macro2::{Punct, Spacing, Span, TokenStream};
use quote::{quote, TokenStreamExt};
use syn::{Ident, LitStr};

#[derive(Debug)]
pub struct Element {
    pub name: String,
    pub attrs: Vec<Attribute>,
    pub children: Vec<Node>,
}

impl Element {
    fn control_attr(&self, name: &str) -> Option<&ControlAttribute> {
        for attr in &self.attrs {
            if let Attribute::Control(control) = attr {
                if control.name == name {
                    return Some(control);
                }
            }
        }

        None
    }

    fn normal_attr(&self, name: &str) -> Option<&NormalAttribute> {
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
        if let Some(attr) = self.normal_attr("name") {
            return attr.value.value();
        }

        "default"
    }

    fn component_children(
        &self,
        idents: &mut Vec<Ident>,
        scopes: &[Ident],
    ) -> (Vec<LitStr>, Vec<TokenStream>, Vec<TokenStream>) {
        let mut names = vec![];
        let mut templates = vec![];
        let mut other = vec![];

        for child in &self.children {
            let mut ts = TokenStream::new();

            child.tokenize(&mut ts, idents, scopes);

            if let Node::Element(e) = child {
                if e.name == "template" {
                    let mut has_named_template = false;

                    for attr in &e.attrs {
                        if let Attribute::Normal(n) = attr {
                            if n.name.starts_with('#') {
                                names
                                    .push(LitStr::new(n.name.get(1..).unwrap(), Span::call_site()));
                                templates.push(ts.clone());
                                has_named_template = true;
                                break;
                            }
                        }
                    }

                    if !has_named_template {
                        other.push(ts);
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

    fn component_attrs(&self, idents: &mut Vec<Ident>, scopes: &[Ident]) -> Vec<TokenStream> {
        let mut attrs = vec![];

        for attr in &self.attrs {
            let mut tokens = TokenStream::new();

            match attr {
                // TODO: snake case (for normal & variable)
                Attribute::Normal(n) => {
                    tokens.append(Ident::new(&n.name, Span::call_site()));
                    tokens.append(Punct::new(':', Spacing::Alone));
                    n.value.tokenize(&mut tokens, idents, scopes);
                }
                Attribute::Variable(v) => {
                    tokens.append(Ident::new(&v.name, Span::call_site()));
                    tokens.append(Punct::new(':', Spacing::Alone));
                    v.value.tokenize(&mut tokens, idents, scopes);
                }
                Attribute::Dynamic(d) => continue,
                _ => continue,
            }

            attrs.push(tokens);
        }

        attrs
    }

    fn end_tokens(&self) -> TokenStream {
        if !VOID_TAGS.contains(&self.name.as_str()) {
            let end_tag = LitStr::new(&format!("</{}>", &self.name), Span::call_site());

            quote! {
                write!(f, "{}", #end_tag)?;
            }
        } else {
            quote! {}
        }
    }

    fn children_tokens(&self, idents: &mut Vec<Ident>, scopes: &[Ident]) -> Vec<TokenStream> {
        let mut tokens = vec![];
        let mut iter = self.children.iter();
        let mut child_option = iter.next();

        while child_option.is_some() {
            let child = child_option.unwrap();

            if let Node::Element(e) = child {
                if e.control_attr("if").is_some() {
                    let mut after_if = vec![child];
                    let mut next = iter.next();
                    let (mut has_else, mut has_else_if) = (false, false);

                    while next.is_some() {
                        let sibling = next.unwrap();

                        if let Node::Element(e) = sibling {
                            // If element has `else`, Mark the children to be cleaned
                            if e.control_attr("else").is_some() {
                                after_if.push(sibling);
                                has_else = true;
                                child_option = iter.next();
                                break;
                            }

                            // If element has `else-if`, mark the children to be cleaned even though we have no `else`
                            if e.control_attr("else-if").is_some() {
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
                        let mut ts = TokenStream::new();

                        i.tokenize(&mut ts, idents, scopes);
                        tokens.push(ts);
                    }

                    // If at the end, break out
                    if next.is_none() {
                        break;
                    }

                    continue;
                }

                if e.control_attr("else").is_some() || e.control_attr("else-if").is_some() {
                    // TODO:(view:err) Show the error position
                    // panic!("expected `!if` element before `!else` or `!else-if`");
                }
            }

            let mut ts = TokenStream::new();

            child.tokenize(&mut ts, idents, scopes);
            tokens.push(ts);
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
    fn tokenize(&self, tokens: &mut TokenStream, idents: &mut Vec<Ident>, scopes: &[Ident]) {
        let tag_pieces: Vec<&str> = self.name.split(':').collect();
        let mut new_scopes = scopes.to_vec();

        // Check for loop to see what variables are defined for this loop (`scopes`)
        if let Some(attr_for) = self.control_attr("for") {
            if let Code::For(for_) = &attr_for.value {
                let mut declared = for_.declared();
                new_scopes.append(&mut declared);
            }
        }

        let mut elem = if tag_pieces.len() == 1 && is_reserved_tag(&self.name) {
            let start_tag = LitStr::new(&format!("<{}", &self.name), Span::call_site());
            let attrs: Vec<TokenStream> = self
                .attrs
                .iter()
                .map(|x| {
                    let mut ts = TokenStream::new();

                    x.tokenize(&mut ts, idents, &new_scopes);
                    ts
                })
                .collect();
            let children = self.children_tokens(idents, &new_scopes);
            let end_tokens = self.end_tokens();

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
            let attrs = self.component_attrs(idents, &new_scopes);
            let (names, templates, children) = self.component_children(idents, &new_scopes);

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
                    #(#attrs),*
                })?;
            }
        };

        elem = if let Some(r_for) = self.control_attr("for") {
            // For loop
            let mut for_expr = TokenStream::new();
            r_for.value.tokenize(&mut for_expr, idents, scopes);

            quote! {
                for #for_expr {
                    #elem
                }
            }
        } else if let Some(r_if) = self.control_attr("if") {
            // If condition
            let mut if_expr = TokenStream::new();
            r_if.value.tokenize(&mut if_expr, idents, scopes);

            quote! {
                if #if_expr {
                    #elem
                }
            }
        } else if let Some(r_else_if) = self.control_attr("else-if") {
            // Else If condition
            let mut if_expr = TokenStream::new();
            r_else_if.value.tokenize(&mut if_expr, idents, scopes);

            quote! {
                else if #if_expr {
                    #elem
                }
            }
        } else if self.control_attr("else").is_some() {
            // Else condition
            quote! {
                else {
                    #elem
                }
            }
        } else {
            elem
        };

        tokens.append_all(elem);
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
