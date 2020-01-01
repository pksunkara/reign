use crate::templates::html_regex;
use inflector::cases::pascalcase::to_pascal_case;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::{env, path::PathBuf};
use syn::{DeriveInput, Ident};

// TODO: No need of this when renaming _layouts dir to layouts
pub(super) fn layouts() -> TokenStream {
    let mut dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let mut result = vec![];

    dir.push("src");
    dir.push("views");
    dir.push("_layouts");

    for entry in dir.read_dir().expect("reading layouts dir failed") {
        if let Ok(entry) = entry {
            let file_name_os_str = entry.file_name();
            let file_name = file_name_os_str.to_str().unwrap();

            if !html_regex().is_match(file_name) {
                continue;
            }

            let cased = to_pascal_case(file_name.trim_end_matches(".html"));
            let ident = Ident::new(&cased, Span::call_site());
            let file_name_str = format!("_layouts/{}", file_name);

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
