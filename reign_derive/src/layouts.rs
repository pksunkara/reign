use proc_macro2::TokenStream;
use quote::quote;
use regex::Regex;
use std::{env, path::PathBuf};
use syn::{DeriveInput, Item, ItemMod};

static LAYOUT_PANIC: &'static str =
    "'layouts' attribute macro is allowed only on 'pub mod layouts'";

pub fn layouts_attribute(item: Item) -> TokenStream {
    match item {
        Item::Mod(ItemMod { ident, content, .. }) => {
            let content_items = match content {
                Some((_, items)) => items,
                None => vec![],
            };

            if ident != "layouts" {
                panic!(LAYOUT_PANIC);
            }

            let mut layouts = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
            let html = Regex::new(r"([a-zA-Z][a-zA-Z0-9_])*\.html$").unwrap();
            let mut result = vec![];

            layouts.push("src");
            layouts.push("views");
            layouts.push("layouts");

            for entry in layouts.read_dir().expect("Reading layouts dir failed") {
                if let Ok(entry) = entry {
                    let file_name = entry.file_name();
                    let file_name_str = format!("layouts/{}", file_name.to_str().unwrap());

                    result.push(quote! {
                        #[derive(Debug, Default, Layout, Template)]
                        #[template(path = #file_name_str)]
                        pub struct LayoutApplication {
                            content: String,
                        }
                    });
                }
            }

            quote! {
                pub mod layouts {
                    use ::askama::Template;
                    use ::reign::derives::Layout;

                    #(#result)*

                    #(#content_items)*
                }
            }
        }
        _ => panic!(LAYOUT_PANIC),
    }
}

pub fn layout_derive(input: DeriveInput) -> TokenStream {
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
