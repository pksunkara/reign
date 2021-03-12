use crate::{
    server::write::{write_file, write_manifest},
    INTERNAL_ERR,
};

use inflector::cases::pascalcase::to_pascal_case;
use proc_macro2::{Ident, Span};
use quote::quote;
use reign_task::Error;
use reign_view::common::{recurse, Manifest, FILE_REGEX, FOLDER_REGEX};

use std::path::{Path, PathBuf};

pub fn parse(views: &Path) -> Result<(), Error> {
    let manifest = parse_all_views(views)?;

    write_manifest(views, &manifest)?;
    Ok(())
}

fn parse_all_views(views: &Path) -> Result<Manifest, Error> {
    let mut manifest = Manifest::new();

    recurse(
        &views,
        "",
        &mut manifest,
        |ident, _| Ok(quote! { pub mod #ident; }),
        |path, file_base_name, file| {
            write_file(
                &path.join(format!("{}.rs", file_base_name)),
                file.to_string(),
            )?;

            let file_name = Ident::new(file_base_name, Span::call_site());
            let ident = Ident::new(&to_pascal_case(file_base_name), Span::call_site());

            Ok(quote! {
                mod #file_name;
                pub use #file_name::#ident;
            })
        },
        |path, views| {
            write_file(&path.join("mod.rs"), quote! { #(#views)* }.to_string())?;

            Ok(vec![])
        },
    )?;

    Ok(manifest)
}

pub fn is_view_folder(full_path: &Path, paths: &[PathBuf]) -> bool {
    paths.iter().any(|path| {
        path.strip_prefix(full_path)
            .expect(INTERNAL_ERR)
            .components()
            .map(|c| c.as_os_str().to_string_lossy())
            .all(|x| FOLDER_REGEX.is_match(&x))
    })
}

pub fn has_any_view_files(full_path: &Path, paths: &[PathBuf]) -> bool {
    let mut has = false;

    for path in paths {
        if let Some((last, view)) = path
            .strip_prefix(full_path)
            .expect(INTERNAL_ERR)
            .components()
            .map(|c| c.as_os_str().to_string_lossy())
            .collect::<Vec<_>>()
            .split_last()
        {
            if FILE_REGEX.is_match(&last) && view.iter().all(|x| FOLDER_REGEX.is_match(&x)) {
                has = true;
            }
        }
    }

    has
}
