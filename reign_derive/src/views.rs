use heck::CamelCase;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use regex::Regex;
use std::{env, path::PathBuf};
use syn::{
    parse::{Parse, ParseStream, Result},
    Ident,
};

pub(super) struct Views {
    folder: Ident,
}

impl Parse for Views {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(Views {
            folder: input.parse()?,
        })
    }
}

pub(super) fn views(input: Views) -> TokenStream {
    let folder = input.folder.to_string();

    let mut dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let html_regex = Regex::new(r"^([a-zA-Z][a-zA-Z0-9_]*)\.html$").unwrap();
    let replacer_regex = Regex::new(r"\.html$").unwrap();
    let mut result = vec![];

    dir.push("src");
    dir.push("views");
    dir.push(&folder);

    for entry in dir.read_dir().expect("reading views dir failed") {
        if let Ok(entry) = entry {
            let file_name_os_str = entry.file_name();
            let file_name = file_name_os_str.to_str().unwrap();

            if !html_regex.is_match(file_name) {
                continue;
            }

            let cased = replacer_regex.replace(file_name, "").to_camel_case();
            let ident = Ident::new(&format!("View{}", cased), Span::call_site());
            let file_name_str = format!("{}/{}", &folder, file_name);

            // TODO: Read template and auto declare needed fields in struct
            result.push(quote! {
                #[derive(Debug, Template)]
                #[template(path = #file_name_str)]
                struct #ident {}
            });
        }
    }

    quote! {
        use ::askama::Template;

        #(#result)*
    }
}
