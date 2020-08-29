use crate::utils::{
    self,
    term::{RED_BOLD, TERM_ERR},
    INTERNAL_ERR,
};
use clap::Clap;
use inflector::cases::pascalcase::to_pascal_case;
use notify::{
    event::{CreateKind, DataChange, EventKind, ModifyKind, RemoveKind},
    Error, Event, RecommendedWatcher, RecursiveMode, Watcher,
};
use proc_macro2::{Ident, Span};
use quote::quote;
use regex::Regex;
use reign_view::common::{recurse, FILE_REGEX, FOLDER_REGEX};
use std::{
    collections::HashMap,
    fs::write,
    path::{Path, PathBuf},
    process::Command,
};

/// Start the Reign server
#[derive(Debug, Clap)]
pub struct Server {
    // TODO:(cli) view directory
}

impl Server {
    pub fn run(&self) -> utils::Result {
        let cargo = Path::new("Cargo.toml");

        if let Err(e) = cargo.canonicalize() {
            TERM_ERR.write_line(&format!(
                "    {} {}",
                RED_BOLD.apply_to("error"),
                "reading Cargo.toml"
            ))?;
            eprintln!("{}", e);
        }

        let views = Path::new("src").join("views");

        if let Err(e) = views.canonicalize() {
            TERM_ERR.write_line(&format!(
                "    {} {}",
                RED_BOLD.apply_to("error"),
                "reading src/views"
            ))?;
            eprintln!("{}", e);
        }

        let mut watcher: RecommendedWatcher =
            Watcher::new_immediate(|res: Result<Event, Error>| match res {
                Ok(event) => match event.kind {
                    EventKind::Modify(ModifyKind::Data(DataChange::Content))
                        if check_regex(&event.paths, &FILE_REGEX) =>
                    {
                        println!("changed content {:?}", event)
                    }
                    EventKind::Create(CreateKind::Folder)
                        if check_regex(&event.paths, &FOLDER_REGEX) =>
                    {
                        println!("created folder")
                    }
                    EventKind::Create(CreateKind::File)
                        if check_regex(&event.paths, &FILE_REGEX) =>
                    {
                        println!("created file")
                    }
                    EventKind::Remove(RemoveKind::Folder)
                        if check_regex(&event.paths, &FOLDER_REGEX) =>
                    {
                        println!("removed folder")
                    }
                    EventKind::Remove(RemoveKind::File)
                        if check_regex(&event.paths, &FILE_REGEX) =>
                    {
                        println!("removed file")
                    }
                    // TODO:(cli) Rename folder/file
                    _ => {}
                },
                Err(e) => println!("watch error: {:?}", e),
            })?;

        watcher.watch(&views, RecursiveMode::Recursive)?;

        let mut map = HashMap::new();

        recurse(
            &views,
            "",
            &mut map,
            |ident, _| {
                quote! { pub mod #ident; }
            },
            |path, file_base_name, file| {
                let new_path = path.join(format!("{}.rs", file_base_name));

                write(&new_path, file.to_string()).expect(INTERNAL_ERR);
                fmt(&new_path).expect(INTERNAL_ERR);

                let file_name = Ident::new(file_base_name, Span::call_site());
                let ident = Ident::new(&to_pascal_case(file_base_name), Span::call_site());

                quote! {
                    mod #file_name;
                    pub use #file_name::#ident;
                }
            },
            |path, views| {
                let new_path = path.join("mod.rs");

                write(&new_path, quote! { #(#views)* }.to_string()).expect(INTERNAL_ERR);
                fmt(&new_path).expect(INTERNAL_ERR);

                vec![]
            },
        );

        write(
            views.join("_manifest.json"),
            serde_json::to_string(&map).expect(INTERNAL_ERR),
        )?;

        self.build();
        Ok(())
    }

    fn build(&self) {
        Command::new("cargo")
            .args(&["build", "--features", "reign/hot-reload"])
            .status()
            .unwrap();

        Command::new("cargo")
            .args(&["run", "--features", "reign/hot-reload"])
            .status()
            .unwrap();
    }
}

fn fmt(path: &Path) -> utils::Result {
    Command::new("rustfmt").args(&[path]).status()?;
    Ok(())
}

fn check_regex(paths: &[PathBuf], regex: &Regex) -> bool {
    if paths.len() != 1 {
        return false;
    }

    if let Some(file_name) = paths.get(0).expect(INTERNAL_ERR).file_name() {
        regex.is_match(&file_name.to_string_lossy())
    } else {
        false
    }
}
