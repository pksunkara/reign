use heck::CamelCase;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use regex::Regex;
use std::{env, path::PathBuf};
use syn::{DeriveInput, Ident};

pub(super) fn layouts() -> TokenStream {
    let mut layouts = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let html_regex = Regex::new(r"^([a-zA-Z][a-zA-Z0-9_]*)\.html$").unwrap();
    let replacer_regex = Regex::new(r"\.html$").unwrap();
    let mut result = vec![];

    layouts.push("src");
    layouts.push("views");
    layouts.push("_layouts");

    for entry in layouts.read_dir().expect("reading layouts dir failed") {
        if let Ok(entry) = entry {
            let file_name_os_str = entry.file_name();
            let file_name = file_name_os_str.to_str().unwrap();

            if !html_regex.is_match(file_name) {
                continue;
            }

            let cased = replacer_regex.replace(file_name, "").to_camel_case();
            let ident = Ident::new(&cased, Span::call_site());
            let file_name_str = format!("_layouts/{}", file_name);

            // TODO: Read template and auto declare needed fields in struct
            result.push(quote! {
                #[derive(Debug, Default, Layout, Template)]
                #[template(path = #file_name_str)]
                pub struct #ident {
                    content: String,
                }
            });
        }
    }

    quote! {
        use ::askama::Template;
        use ::reign::prelude::Layout;

        #(#result)*
    }
}

pub(super) fn layout_derive(input: DeriveInput) -> TokenStream {
    let ident = &input.ident;

    quote! {
        impl ::reign::view::Layout for #ident {
            fn content(self, content: String) -> Self {
                let mut ret = self;

                ret.content = content;
                ret
            }
        }
    }
}
