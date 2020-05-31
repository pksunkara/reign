use super::consts::*;
use super::{
    attribute::{ControlAttribute, NormalAttribute},
    tag_name_regex, Attribute, Code, Error, Node, Parse, ParseStream, Tokenize, ViewFields,
};
use inflector::cases::{pascalcase::to_pascal_case, snakecase::to_snake_case};
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

    fn slot_name(&self) -> String {
        if let Some(attr) = self.normal_attr("name") {
            if let Some(name) = attr.value.value() {
                return name;
            } else {
                // TODO:(view:err) Show the error position
                panic!("slot name should not have expression");
            }
        }

        "default".to_string()
    }

    fn template_name(&self) -> Option<String> {
        if self.name == "template" {
            for attr in &self.attrs {
                if let Attribute::Normal(n) = attr {
                    if n.name.starts_with('#') {
                        return Some(n.name.clone());
                    }
                }
            }
        }

        None
    }

    fn templates(
        &self,
        idents: &mut ViewFields,
        scopes: &ViewFields,
    ) -> (Vec<LitStr>, Vec<TokenStream>) {
        let mut names = vec![];
        let mut templates = vec![];

        for child in &self.children {
            if let Node::Element(e) = child {
                if let Some(name) = e.template_name() {
                    let name = name.get(1..).unwrap();
                    let mut ts = TokenStream::new();

                    child.tokenize(&mut ts, idents, scopes);

                    names.push(LitStr::new(name, Span::call_site()));
                    templates.push(ts);
                }
            }
        }

        (names, templates)
    }

    // TODO: Build a DAG out of the views, and use default() if the attrs are not defined
    // It would be even better if we could compile each html file into `.rs` file and use
    // it to speed up compile times.
    //
    // If we have the DAG, and we see an html file was changed, we rebuild that view, and
    // if any of it fields have changed, we need to go up in the DAG and recompile all the
    // views that depend on this.
    //
    // After having DAG, we can also look into intelligently forwarding the types of the
    // view fields into each components.
    fn component_attrs(&self, idents: &mut ViewFields, scopes: &ViewFields) -> Vec<TokenStream> {
        let mut attrs = vec![];

        for attr in &self.attrs {
            let mut tokens = TokenStream::new();

            match attr {
                Attribute::Normal(n) => {
                    tokens.append(Ident::new(&to_snake_case(&n.name), Span::call_site()));
                    tokens.append(Punct::new(':', Spacing::Alone));
                    n.value.tokenize(&mut tokens, idents, scopes);
                }
                Attribute::Variable(v) => {
                    tokens.append(Ident::new(&to_snake_case(&v.name), Span::call_site()));
                    tokens.append(Punct::new(':', Spacing::Alone));
                    v.value.tokenize(&mut tokens, idents, scopes);
                }
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

    fn attrs_tokens(&self, idents: &mut ViewFields, scopes: &ViewFields) -> Vec<TokenStream> {
        self.attrs
            .iter()
            .map(|x| {
                let mut ts = TokenStream::new();

                x.tokenize(&mut ts, idents, &scopes);
                ts
            })
            .collect()
    }

    fn children_tokens(&self, idents: &mut ViewFields, scopes: &ViewFields) -> Vec<TokenStream> {
        let mut tokens = vec![];
        let mut iter = self.children.iter();
        let mut child_option = iter.next();

        while child_option.is_some() {
            let child = child_option.unwrap();

            if let Node::Element(e) = child {
                if e.template_name().is_some() {
                    child_option = iter.next();
                    continue;
                }

                if e.control_attr("if").is_some() {
                    let mut after_if = vec![child];
                    let mut next = iter.next();
                    let (mut has_else, mut has_else_if) = (false, false);

                    while next.is_some() {
                        let sibling = next.unwrap();

                        if let Node::Element(e) = sibling {
                            if e.template_name().is_some() {
                                next = iter.next();
                                continue;
                            }

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
                    panic!("expected `!if` element before `!else` or `!else-if`");
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
    #[allow(clippy::cognitive_complexity)]
    fn tokenize(&self, tokens: &mut TokenStream, idents: &mut ViewFields, scopes: &ViewFields) {
        let tag_pieces: Vec<&str> = self.name.split(':').collect();
        let mut new_scopes = scopes.clone();

        // Check for loop to see what variables are defined for this loop (`scopes`)
        if let Some(attr_for) = self.control_attr("for") {
            if let Code::For(for_) = &attr_for.value {
                new_scopes.append(for_.declared());
            }
        }

        let mut elem = if self.name == "template" {
            let children = self.children_tokens(idents, &new_scopes);

            quote! {
                #(#children)*
            }
        } else if self.name == "slot" {
            let name = LitStr::new(&self.slot_name(), Span::call_site());

            quote! {
                self._slots.render(f, #name)?;
            }
        } else if tag_pieces.len() == 1 && is_reserved_tag(&self.name) {
            let start_tag = LitStr::new(&format!("<{}", &self.name), Span::call_site());
            let attrs = self.attrs_tokens(idents, &new_scopes);
            let children = self.children_tokens(idents, &new_scopes);
            let end_tokens = self.end_tokens();

            quote! {
                write!(f, "{}", #start_tag)?;
                #(#attrs)*
                write!(f, ">")?;
                #(#children)*
                #end_tokens
            }
        } else {
            let path = convert_tag_name(tag_pieces);
            let attrs = self.component_attrs(idents, &new_scopes);
            let (names, templates) = self.templates(idents, &new_scopes);
            let children = self.children_tokens(idents, &new_scopes);

            quote! {
                write!(f, "{}", crate::views::#(#path)::* {
                    _slots: ::reign::view::Slots {
                        templates: ::reign::view::maplit::hashmap!{
                            #(#names => ::reign::view::slot_render(|f: &mut dyn std::fmt::Write| {
                                #templates
                                Ok(())
                            })),*
                        },
                        children: ::reign::view::slot_render(|f: &mut dyn std::fmt::Write| {
                            #(#children)*
                            Ok(())
                        }),
                        phantom: ::std::marker::PhantomData,
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
        .map(|t| Ident::new(&to_snake_case(t), Span::call_site()))
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
