use crate::router::ty::subty_if_name;
use proc_macro2::TokenStream;
use proc_macro_error::{abort, abort_call_site};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    token::{At, Brace, Bracket, Colon, Div, Question, Star},
    Block, Expr, ExprMacro, Ident, LitStr, Macro, Stmt, Type,
};

#[derive(Clone)]
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

    pub fn ty(&self) -> TokenStream {
        let ty = if let Some(ty) = &self.ty {
            quote!(#ty)
        } else {
            quote!(String)
        };

        let ty = if self.glob { quote!(Vec<#ty>) } else { ty };

        if self.optional {
            quote!(Option<#ty>)
        } else {
            ty
        }
    }

    fn parse(&mut self, input: ParseStream) -> Result<()> {
        if input.peek(Colon) {
            input.parse::<Colon>()?;
            self.ty = Some(input.parse()?);
        }

        if input.peek(At) {
            input.parse::<At>()?;
            self.regex = Some(input.parse()?);
        }

        if let Some(ty) = self.ty.clone() {
            if let Some(ty) = subty_if_name(ty.clone(), "Vec") {
                self.glob = true;
                self.ty = Some(ty);
            } else if let Some(ty) = subty_if_name(ty, "Option") {
                self.optional = true;

                if let Some(ty) = subty_if_name(ty.clone(), "Vec") {
                    self.glob = true;
                    self.ty = Some(ty);
                } else {
                    self.ty = Some(ty);
                }
            }
        }

        Ok(())
    }
}

pub enum PathSegment {
    Static(LitStr),
    Dynamic(Box<PathSegmentDynamic>),
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
                dynamic.parse(input)?;
            }

            Ok(PathSegment::Dynamic(Box::new(dynamic)))
        }
    }
}

impl ToTokens for PathSegment {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            PathSegment::Static(s) => {
                s.to_tokens(tokens);
            }
            PathSegment::Dynamic(d) => {
                d.ident.to_tokens(tokens);
                Colon::default().to_tokens(tokens);
                tokens.append_all(d.ty());

                if let Some(regex) = &d.regex {
                    tokens.append_all(quote!(@ #regex));
                }
            }
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
                if input.peek(Brace) || input.peek(Bracket) || input.is_empty() {
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
}
