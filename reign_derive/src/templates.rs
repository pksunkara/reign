use inflector::cases::snakecase::to_snake_case;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use regex::Regex;
use reign_view::parse::parse;
use std::env;
use std::fs::read_to_string;
use std::path::PathBuf;

use syn::{
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    token::Comma,
    Ident, LitStr,
};

pub(super) struct Templates {
    paths: Punctuated<LitStr, Comma>,
}

impl Parse for Templates {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Templates {
            paths: input.parse_terminated(|i| i.parse::<LitStr>())?,
        })
    }
}

pub(crate) fn file_regex() -> Regex {
    Regex::new(r"^([[:alpha:]]([[:word:]]*[[:alnum:]])?)\.html$").unwrap()
}

pub(crate) fn folder_regex() -> Regex {
    Regex::new(r"^([[:alpha:]]([[:word:]]*[[:alnum:]])?)").unwrap()
}

fn recurse(path: &PathBuf) -> Vec<proc_macro2::TokenStream> {
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
                let sub_views = recurse(&new_path);

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

            let cased = to_snake_case(file_name.trim_end_matches(".html"));
            let ident = Ident::new(&cased, Span::call_site());

            let node = parse(read_to_string(new_path).unwrap()).unwrap();

            views.push(quote! {
                pub fn #ident() -> String {
                    String::from("")
                }
            });
        }
    }

    views
}

pub(super) fn templates(input: Templates) -> TokenStream {
    let mut dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    for (_, i) in input.paths.into_iter().enumerate() {
        dir.push(i.value());
    }

    let views = recurse(&dir);

    quote! {
        pub mod views {
            #(#views)*
        }
    }
}
