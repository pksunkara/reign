use crate::{
    server::write::{write_file, write_manifest},
    utils::Result,
};
use inflector::cases::pascalcase::to_pascal_case;
use proc_macro2::{Ident, Span};
use quote::quote;
use reign_view::common::{recurse, Manifest};
use std::path::Path;

pub fn build(views: &Path) -> Result {
    let manifest = build_all_views(views)?;

    write_manifest(views, &manifest)?;
    Ok(())
}

pub fn build_all_views(views: &Path) -> Result<Manifest> {
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
