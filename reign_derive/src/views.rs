use heck::CamelCase;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use regex::Regex;
use std::{env, path::PathBuf};
use syn::{Ident, Item, ItemMod};

static VIEWS_MACRO_PANIC: &str = "'views' attribute macro is allowed only on 'mod views'";

pub fn views_attribute(attr: &str, item: Item) -> TokenStream {
    match item {
        Item::Mod(ItemMod { ident, content, .. }) => {
            let content_items = match content {
                Some((_, items)) => items,
                None => vec![],
            };

            if ident != "views" {
                panic!(VIEWS_MACRO_PANIC);
            }

            let mut dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
            let html_regex = Regex::new(r"^([a-zA-Z][a-zA-Z0-9_]*)\.html$").unwrap();
            let replacer_regex = Regex::new(r"\.html$").unwrap();
            let mut result = vec![];

            dir.push("src");
            dir.push("views");
            dir.push(attr);

            for entry in dir.read_dir().expect("reading views dir failed") {
                if let Ok(entry) = entry {
                    let file_name_os_str = entry.file_name();
                    let file_name = file_name_os_str.to_str().unwrap();

                    if !html_regex.is_match(file_name) {
                        continue;
                    }

                    let cased = replacer_regex.replace(file_name, "").to_camel_case();
                    let ident = Ident::new(&format!("View{}", cased), Span::call_site());
                    let file_name_str = format!("{}/{}", attr, file_name);

                    // TODO: Read template and auto declare needed fields in struct
                    result.push(quote! {
                        #[derive(Debug, Template)]
                        #[template(path = #file_name_str)]
                        pub(super) struct #ident {}
                    });
                }
            }

            quote! {
                mod views {
                    use ::askama::Template;

                    #(#result)*

                    #(#content_items)*
                }
            }
        }
        _ => panic!(VIEWS_MACRO_PANIC),
    }
}
