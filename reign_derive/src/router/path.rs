use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    token::{At, Brace, Bracket, Div, Question, Star},
    Ident, LitStr,
};

#[derive(Clone)]
pub struct PathSegmentDynamic {
    pub ident: Ident,
    pub optional: bool,
    pub glob: bool,
    pub regex: Option<LitStr>,
}

impl PathSegmentDynamic {
    fn new(ident: Ident) -> Self {
        Self {
            ident,
            optional: false,
            glob: false,
            regex: None,
        }
    }
}

impl Parse for PathSegmentDynamic {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut ret = Self::new(input.parse()?);

        while input.peek(Question) || input.peek(Star) || input.peek(At) {
            if input.peek(Question) {
                input.parse::<Question>()?;
                ret.optional = true;
            }

            if input.peek(Star) {
                input.parse::<Star>()?;
                ret.glob = true;
            }

            if input.peek(At) {
                input.parse::<At>()?;
                ret.regex = Some(input.parse()?);
            }
        }

        Ok(ret)
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
            Ok(PathSegment::Static(lit))
        } else {
            let dynamic: PathSegmentDynamic = input.parse()?;
            Ok(PathSegment::Dynamic(Box::new(dynamic)))
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

pub fn path(input: Path) -> TokenStream {
    input.segments.into_iter().fold(
        quote! {
            ::reign::router::Path::new()
        },
        |tokens, segment| match segment {
            PathSegment::Static(lit) => quote! { #tokens.path(#lit) },
            PathSegment::Dynamic(d) => {
                let name = LitStr::new(&d.ident.to_string(), d.ident.span());
                let method = Ident::new(
                    &format!(
                        "param{}{}",
                        if d.optional { "_opt" } else { "" },
                        if d.regex.is_some() || d.glob {
                            "_regex"
                        } else {
                            ""
                        }
                    ),
                    d.ident.span(),
                );

                let regex = if let Some(regex) = d.regex {
                    quote! {
                        , #regex
                    }
                } else if d.glob {
                    quote! {
                        , ".+"
                    }
                } else {
                    quote! {}
                };

                quote! { #tokens.#method(#name #regex) }
            }
        },
    )
}
