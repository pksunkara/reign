use crate::{utils::Options, INTERNAL_ERR};
use inflector::cases::pascalcase::to_pascal_case;
use once_cell::sync::OnceCell;
use proc_macro2::{Span, TokenStream};
use proc_macro_error::abort_call_site;
use quote::quote;
#[cfg(feature = "hot-reload")]
use serde_json::from_str;
#[cfg(feature = "hot-reload")]
use std::fs::read_to_string;
use std::{collections::HashMap, env, path::PathBuf};
use syn::{
    parse::{Parse, ParseStream, Result},
    parse_str,
    punctuated::Punctuated,
    token::{Colon2, Comma},
    Expr, Ident, LitStr,
};

#[cfg(feature = "hot-reload")]
static DIR: OnceCell<PathBuf> = OnceCell::new();
static IDENTMAP: OnceCell<HashMap<String, Vec<(String, bool)>>> = OnceCell::new();

// TODO: derive: Options after the paths (including changing `crate::views`)
// Can't use parse_separated_non_empty here
pub struct Views {
    paths: Punctuated<LitStr, Comma>,
}

impl Parse for Views {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Views {
            paths: input.parse_terminated(|i| i.parse::<LitStr>())?,
        })
    }
}

pub struct Render {
    path: Punctuated<Ident, Colon2>,
    options: Options,
}

impl Render {
    fn id(&self) -> String {
        self.parts().join(":")
    }

    fn parts(&self) -> Vec<String> {
        self.path.iter().map(|i| format!("{}", i)).collect()
    }
}

impl Parse for Render {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Render {
            path: Punctuated::<Ident, Colon2>::parse_separated_nonempty(input)?,
            options: input.parse()?,
        })
    }
}

fn get_dir(input: Views) -> PathBuf {
    let mut dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    for i in input.paths.into_iter() {
        dir.push(i.value());
    }

    dir
}

#[cfg(feature = "hot-reload")]
pub fn views(input: Views) -> TokenStream {
    let dir = get_dir(input);

    DIR.set(dir).expect(INTERNAL_ERR);

    quote! {
        pub mod views;
    }
}

#[cfg(not(feature = "hot-reload"))]
pub fn views(input: Views) -> TokenStream {
    let dir = get_dir(input);
    let mut map = HashMap::new();

    let views = reign_view::common::recurse(
        &dir,
        "",
        &mut map,
        |ident, files| {
            Ok(quote! {
                pub mod #ident {
                    #(#files)*
                }
            })
        },
        |_, _, file| Ok(file),
        |_, views| Ok(views),
    )
    .expect(INTERNAL_ERR);

    IDENTMAP.set(map).expect(INTERNAL_ERR);

    quote! {
        pub mod views {
            #(#views)*
        }
    }
}

fn read_manifest() {
    #[cfg(feature = "hot-reload")]
    if let None = IDENTMAP.get() {
        let dir = DIR.get().expect(INTERNAL_ERR).clone();

        let manifest = read_to_string(dir.join("_manifest.json"));

        if manifest.is_err() {
            abort_call_site!("expected _manifest.json to exist and readable");
        }

        IDENTMAP
            .set(from_str(&manifest.expect(INTERNAL_ERR)).expect(INTERNAL_ERR))
            .expect(INTERNAL_ERR);
    }
}

fn view_path(input: &Render) -> TokenStream {
    read_manifest();

    let parts = input.parts();
    let (last, elements) = parts.split_last().unwrap();

    let view = Ident::new(&to_pascal_case(last), Span::call_site());
    let path: Vec<Ident> = elements
        .iter()
        .map(|x| Ident::new(x, Span::call_site()))
        .collect();

    quote! {
        #(#path::)*#view
    }
}

fn capture(input: &Render) -> TokenStream {
    let path = view_path(input);
    let value = IDENTMAP.get().expect(INTERNAL_ERR).get(&input.id());

    if value.is_none() {
        abort_call_site!("expected a path referencing to a view file");
    }

    let idents: Vec<TokenStream> = value
        .expect(INTERNAL_ERR)
        .iter()
        .map(|x| {
            let ident = Ident::new(&x.0, Span::call_site());

            let rest = if x.1 {
                quote! {}
            } else {
                quote! {
                    : #ident.as_ref()
                }
            };

            quote! {
                #ident#rest
            }
        })
        .collect();

    quote! {
        crate::views::#path {
            _slots: ::reign::view::Slots::default(),
            #(#idents),*
        }
    }
}

pub fn render(mut input: Render) -> TokenStream {
    let capture = capture(&input);

    let status: Expr = input
        .options
        .remove("status")
        .unwrap_or_else(|| parse_str("200").unwrap());

    if cfg!(feature = "view-backend") {
        quote! {
            ::reign::view::render(#capture, #status)
        }
    } else {
        quote! {
            format!("{}", #capture)
        }
    }
}
