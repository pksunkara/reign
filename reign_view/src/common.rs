use crate::{
    parse::{parse, tokenize},
    INTERNAL_ERR,
};
use inflector::cases::pascalcase::to_pascal_case;
use once_cell::sync::Lazy;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use regex::Regex;
use std::{collections::HashMap, fs::read_to_string, path::Path};

pub static FILE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^([[:alpha:]]([[:word:]]*[[:alnum:]])?)\.html$").expect(INTERNAL_ERR)
});
pub static FOLDER_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^([[:alpha:]]([[:word:]]*[[:alnum:]])?)").expect(INTERNAL_ERR));

pub fn tokenize_view(path: &Path, file_base_name: &str) -> (TokenStream, Vec<(Ident, bool)>) {
    let cased = to_pascal_case(file_base_name);
    let ident = Ident::new(&cased, Span::call_site());

    let (tokens, idents, types) = tokenize(
        parse(
            read_to_string(path)
                .expect(INTERNAL_ERR)
                .replace("\r\n", "\n"),
        )
        .expect(INTERNAL_ERR),
    );

    let new_idents: Vec<Ident> = idents.iter().map(|x| x.0.clone()).collect();

    (
        quote! {
            pub struct #ident<'a> {
                pub _slots: ::reign::view::Slots<'a>,
                #(pub #new_idents: #types),*
            }

            #[allow(unused_variables)]
            impl<'a> std::fmt::Display for #ident<'a> {
                fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                    #tokens
                    Ok(())
                }
            }
        },
        idents,
    )
}

pub fn recurse<O, I, P>(
    path: &Path,
    relative_path: &str,
    map: &mut HashMap<String, Vec<(String, bool)>>,
    folder_hook: O,
    file_hook: I,
    path_hook: P,
) -> Vec<TokenStream>
where
    O: Fn(Ident, Vec<TokenStream>) -> TokenStream + Copy,
    I: Fn(&Path, &str, TokenStream) -> TokenStream + Copy,
    P: Fn(&Path, Vec<TokenStream>) -> Vec<TokenStream> + Copy,
{
    let mut views = vec![];

    for entry in path.read_dir().expect(INTERNAL_ERR) {
        if let Ok(entry) = entry {
            let new_path = entry.path();
            let file_name_os_str = entry.file_name();
            let file_name = file_name_os_str.to_str().expect(INTERNAL_ERR);

            if new_path.is_dir() {
                if !FOLDER_REGEX.is_match(file_name) {
                    continue;
                }

                let ident = Ident::new(file_name, Span::call_site());
                let sub_relative_path = format!("{}:{}", relative_path, file_name);

                let sub_views = recurse(
                    &new_path,
                    &sub_relative_path,
                    map,
                    folder_hook,
                    file_hook,
                    path_hook,
                );

                views.push(folder_hook(ident, sub_views));
                continue;
            }

            if !FILE_REGEX.is_match(file_name) {
                continue;
            }

            let file_base_name = file_name.trim_end_matches(".html");
            let (file_view, idents) = tokenize_view(&new_path, file_base_name);

            let file_key = format!("{}:{}", relative_path, file_base_name)
                .trim_start_matches(':')
                .to_string();

            map.insert(
                file_key,
                idents.iter().map(|x| (format!("{}", x.0), x.1)).collect(),
            );

            views.push(file_hook(path, file_base_name, file_view));
        }
    }

    path_hook(path, views)
}
