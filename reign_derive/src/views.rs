use inflector::cases::pascalcase::to_pascal_case;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use regex::Regex;
use reign_view::parse::{parse, tokenize};
use std::env;
use std::fs::read_to_string;
use std::path::PathBuf;

use syn::{
    parse::{Parse, ParseStream, Result},
    punctuated::Punctuated,
    token::Comma,
    Ident, LitStr,
};

// TODO: Options after the paths
pub(super) struct Views {
    paths: Punctuated<LitStr, Comma>,
}

impl Parse for Views {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Views {
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
                        use ::reign::view::View;

                        #(#sub_views)*
                    }
                });

                continue;
            }

            if !file_regex().is_match(file_name) {
                continue;
            }

            let cased = to_pascal_case(file_name.trim_end_matches(".html"));
            let ident = Ident::new(&cased, Span::call_site());

            let strings = tokenize(parse(read_to_string(new_path).unwrap()).unwrap());

            views.push(quote! {
                pub struct #ident {
                    pub _slots: ::reign::view::Slots,
                }

                impl ::reign::view::View for #ident {
                    fn render(&self, f: &mut dyn std::fmt::Write) -> std::fmt::Result {
                        #strings
                        Ok(())
                    }
                }

                impl std::fmt::Display for #ident {
                    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                        self.render(f)
                    }
                }
            });
        }
    }

    views
}

pub(super) fn views(input: Views) -> TokenStream {
    let mut dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    for (_, i) in input.paths.into_iter().enumerate() {
        dir.push(i.value());
    }

    let views = recurse(&dir);

    quote! {
        pub mod views {
            use ::reign::view::View;

            #(#views)*
        }
    }
}
