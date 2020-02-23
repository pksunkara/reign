use inflector::cases::pascalcase::to_pascal_case;
use lazy_static::lazy_static;
use proc_macro2::{Span, TokenStream};
use proc_macro_error::abort;
use quote::quote;
use regex::Regex;
use reign_view::parse::{parse, tokenize};
use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::sync::Mutex;
use syn::{
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    token::Comma,
    Ident, LitStr,
};

lazy_static! {
    static ref IDENTMAP: Mutex<HashMap<String, Vec<String>>> = Mutex::new(HashMap::new());
}

// TODO: Options after the paths (including changing `crate::views`)
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

fn file_regex() -> Regex {
    Regex::new(r"^([[:alpha:]]([[:word:]]*[[:alnum:]])?)\.html$").unwrap()
}

fn folder_regex() -> Regex {
    Regex::new(r"^([[:alpha:]]([[:word:]]*[[:alnum:]])?)").unwrap()
}

fn recurse(path: &PathBuf, relative_path: &str) -> Vec<proc_macro2::TokenStream> {
    let mut views = vec![];

    for entry in path.read_dir().unwrap() {
        if let Ok(entry) = entry {
            let new_path = entry.path();
            let file_name_os_str = entry.file_name();
            let file_name = file_name_os_str.to_str().unwrap();

            if new_path.is_dir() {
                if !folder_regex().is_match(file_name) {
                    continue;
                }

                let ident = Ident::new(file_name, Span::call_site());
                let sub_relative_path = format!("{}:{}", relative_path, file_name);
                let sub_views = recurse(&new_path, &sub_relative_path);

                views.push(quote! {
                    pub mod #ident {
                        #(#sub_views)*
                    }
                });

                continue;
            }

            if !file_regex().is_match(file_name) {
                continue;
            }

            let file_base_name = file_name.trim_end_matches(".html");
            let cased = to_pascal_case(file_base_name);
            let ident = Ident::new(&cased, Span::call_site());

            let (tokens, idents, types) =
                tokenize(parse(read_to_string(new_path).unwrap()).unwrap());

            let file_key = format!("{}:{}", relative_path, file_base_name)
                .trim_start_matches(':')
                .to_string();

            IDENTMAP
                .lock()
                .unwrap()
                .insert(file_key, idents.iter().map(|x| format!("{}", x)).collect());

            views.push(quote! {
                pub struct #ident<'a> {
                    pub _slots: ::reign::view::Slots<'a>,
                    #(pub #idents: #types),*
                }

                impl<'a> std::fmt::Display for #ident<'a> {
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                        #tokens
                        Ok(())
                    }
                }
            });
        }
    }

    views
}

pub fn views(input: Views) -> TokenStream {
    let mut dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    for i in input.paths.into_iter() {
        dir.push(i.value());
    }

    let views = recurse(&dir, "");

    quote! {
        pub mod views {
            #(#views)*
        }
    }
}

fn view_path(input: LitStr) -> TokenStream {
    let parts: Vec<String> = input.value().split(':').map(|x| x.to_string()).collect();
    let (last, elements) = parts.split_last().unwrap();

    if last == "" {
        abort!(input.span(), "expected a non-empty string");
    }

    let view = Ident::new(&to_pascal_case(last), Span::call_site());
    let path: Vec<Ident> = elements
        .iter()
        .map(|x| Ident::new(x, Span::call_site()))
        .collect();

    quote! {
        #(#path::)*#view
    }
}

fn capture(input: LitStr) -> TokenStream {
    let path = view_path(input.clone());
    let ident_map = IDENTMAP.lock().unwrap();
    let value = ident_map.get(&input.value());

    if value.is_none() {
        abort!(input.span(), "expected a string referencing to a view file");
    }

    let idents: Vec<Ident> = value
        .unwrap()
        .iter()
        .map(|x| Ident::new(x, Span::call_site()))
        .collect();

    quote! {
        crate::views::#path {
            _slots: ::reign::view::Slots::default(),
            #(#idents),*
        }
    }
}

pub fn render(input: LitStr) -> TokenStream {
    let capture = capture(input);

    if cfg!(feature = "views-gotham") {
        quote! {
            ::reign::view::render_gotham(state, #capture)
        }
    } else if cfg!(feature = "views-warp") {
        quote! {
            ::reign::view::render_warp(#capture)
        }
    } else if cfg!(feature = "views-tide") {
        quote! {
            ::reign::view::render_tide(#capture)
        }
    } else if cfg!(feature = "views-actix") {
        quote! {
            ::reign::view::render_actix(#capture)
        }
    } else {
        quote! {
            format!("{}", #capture)
        }
    }
}
