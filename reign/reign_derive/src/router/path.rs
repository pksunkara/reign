use crate::router::ty::subty_if_name;
use proc_macro_error::{abort, abort_call_site};
use syn::{
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    token::{At, Brace, Bracket, Colon, Div, Question, Star},
    Ident, LitStr, Type,
};

pub struct PathSegmentDynamic {
    pub ident: Ident,
    pub optional: bool,
    pub glob: bool,
    pub ty: Option<Type>,
    pub regex: Option<LitStr>,
}

impl PathSegmentDynamic {
    fn new(ident: Ident) -> Self {
        Self {
            ident,
            optional: false,
            glob: false,
            ty: None,
            regex: None,
        }
    }

    pub fn actix(&self) -> String {
        if self.glob {
            format!("{{{}:.+}}", self.ident)
        } else if let Some(regex) = &self.regex {
            format!("{{{}:{}}}", self.ident, regex.value())
        } else {
            format!("{{{}}}", self.ident)
        }
    }

    pub fn gotham(&self) -> String {
        if self.glob {
            "*".to_string()
        } else if let Some(regex) = &self.regex {
            format!(":{}:{}", self.ident, regex.value())
        } else {
            format!(":{}", self.ident)
        }
    }

    pub fn tide(&self) -> String {
        if self.glob {
            format!("*{}", self.ident)
        } else if let Some(regex) = &self.regex {
            abort!(regex.span(), "tide does not support regex path params");
        } else {
            format!(":{}", self.ident)
        }
    }
}

pub enum PathSegment {
    Static(LitStr),
    Dynamic(PathSegmentDynamic),
}

impl Parse for PathSegment {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(LitStr) {
            let lit: LitStr = input.parse()?;
            // TODO:(router) only allow url encoded strings
            Ok(PathSegment::Static(lit))
        } else {
            let mut dynamic = PathSegmentDynamic::new(input.parse()?);

            if input.peek(Question) {
                input.parse::<Question>()?;
                dynamic.optional = true;
            } else if input.peek(Star) {
                input.parse::<Star>()?;
                dynamic.glob = true;

                if input.peek(Question) {
                    input.parse::<Question>()?;
                    dynamic.optional = true;
                }
            } else {
                if input.peek(Colon) {
                    input.parse::<Colon>()?;
                    dynamic.ty = Some(input.parse()?);
                }

                if input.peek(At) {
                    input.parse::<At>()?;
                    dynamic.regex = Some(input.parse()?);
                }

                if let Some(ty) = dynamic.ty.clone() {
                    if let Some(ty) = subty_if_name(ty.clone(), "Vec") {
                        dynamic.glob = true;
                        dynamic.ty = Some(ty);
                    } else if let Some(ty) = subty_if_name(ty.clone(), "Option") {
                        dynamic.optional = true;

                        if let Some(ty) = subty_if_name(ty.clone(), "Vec") {
                            dynamic.glob = true;
                            dynamic.ty = Some(ty);
                        } else {
                            dynamic.ty = Some(ty);
                        }
                    }
                }
            }

            Ok(PathSegment::Dynamic(dynamic))
        }
    }
}

pub struct Path {
    pub segments: Punctuated<PathSegment, Div>,
}

impl Parse for Path {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Path {
            segments: {
                if input.peek(Brace) || input.peek(Bracket) {
                    Punctuated::new()
                } else {
                    Punctuated::parse_separated_nonempty_with(input, |i| i.parse::<PathSegment>())?
                }
            },
        })
    }
}

impl Path {
    pub fn add(paths: &mut Vec<Vec<String>>, val: String) {
        for i in paths {
            i.push(val.clone())
        }
    }

    pub fn optional(paths: &mut Vec<Vec<String>>, val: String) {
        let mut duplicates = paths.clone();

        Self::add(&mut duplicates, val);
        paths.append(&mut duplicates);
    }

    pub fn actix(self, is_scope: bool) -> Vec<String> {
        let mut paths = vec![vec![]];

        for segment in self.segments {
            match segment {
                PathSegment::Static(s) => Self::add(&mut paths, s.value()),
                PathSegment::Dynamic(d) => {
                    let part = d.actix();

                    if d.optional {
                        if is_scope {
                            abort_call_site!(
                                "actix does not support optional path params in scope"
                            );
                        } else {
                            Self::optional(&mut paths, part);
                        }
                    } else {
                        Self::add(&mut paths, part);
                    }
                }
            }
        }

        paths.into_iter().map(|x| x.join("/")).collect()
    }

    pub fn gotham(self, _is_scope: bool) -> Vec<String> {
        let mut paths = vec![vec![]];

        for segment in self.segments {
            match segment {
                PathSegment::Static(s) => Self::add(&mut paths, s.value()),
                PathSegment::Dynamic(d) => {
                    let part = d.gotham();

                    if d.optional {
                        Self::optional(&mut paths, part);
                    } else {
                        Self::add(&mut paths, part);
                    }
                }
            }
        }

        paths.into_iter().map(|x| x.join("/")).collect()
    }

    pub fn tide(self, _is_scope: bool) -> Vec<String> {
        let mut paths = vec![vec![]];

        for segment in self.segments {
            match segment {
                PathSegment::Static(s) => Self::add(&mut paths, s.value()),
                PathSegment::Dynamic(d) => {
                    let part = d.tide();

                    if d.optional {
                        abort_call_site!("tide does not support optional path params")
                    } else {
                        Self::add(&mut paths, part);
                    }
                }
            }
        }

        paths.into_iter().map(|x| x.join("/")).collect()
    }
}
