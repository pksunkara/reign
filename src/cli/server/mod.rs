use crate::{
    server::view::{has_any_view_files, is_view_folder, parse},
    utils::{
        self,
        term::{RED_BOLD, TERM_ERR},
    },
};
use clap::Clap;
use notify::{
    event::{CreateKind, DataChange, EventKind, ModifyKind, RemoveKind},
    Error, Event, RecommendedWatcher, RecursiveMode, Watcher,
};
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    process::Command,
};

mod view;
mod write;

// TODO:(cli) Live reload not seeming to help compile time

// TODO:(cli) view directory
// TODO:(cli) features for using when building/running
// TODO:(cli) debounce events
// TODO:(cli) rename folder/file

/// Start the Reign server
#[derive(Debug, Clap)]
pub struct Server {}

impl Server {
    pub fn run(&self) -> utils::Result {
        let cargo = Path::new("Cargo.toml");

        check_path(cargo, "Cargo.toml")?;

        let src = Path::new("src");
        let views = src.join("views");

        check_path(src, "src")?;
        check_path(&views, "src/views")?;

        // TODO: cli: Maintain dep graph to reload views that depend on views and build
        // just those views instead of rebuilding everything. Also for views that changed
        // maintain manifest such that manifest is updated only if idents changed

        let _view_watcher = self.view_watcher(&views)?;

        Command::new("cargo")
            .args(&["watch", "-x", "run --features reign/hot-reload"])
            .status()?;

        Ok(())
    }

    fn view_watcher(&self, views: &Path) -> utils::Result<RecommendedWatcher> {
        let full_path = views.canonicalize()?;

        parse(views)?;

        let mut watcher: RecommendedWatcher =
            Watcher::new_immediate(move |res: Result<Event, Error>| match res {
                Ok(event) => match event.kind {
                    EventKind::Modify(ModifyKind::Data(DataChange::Content))
                    | EventKind::Create(CreateKind::File)
                    | EventKind::Remove(RemoveKind::File)
                        if has_any_view_files(&full_path, &event.paths) =>
                    {
                        parse(&full_path).unwrap();
                    }
                    EventKind::Remove(RemoveKind::Folder)
                        if is_view_folder(&full_path, &event.paths) =>
                    {
                        parse(&full_path).unwrap();
                    }
                    _ => {}
                },
                Err(e) => println!("watch error: {:?}", e),
            })?;

        watcher.watch(views, RecursiveMode::Recursive)?;
        Ok(watcher)
    }
}

fn check_path(path: &Path, name: &str) -> utils::Result {
    if let Err(e) = path.canonicalize() {
        TERM_ERR.write_line(&format!(
            "    {} reading {}",
            RED_BOLD.apply_to("error"),
            name
        ))?;
        eprintln!("{}", e);
    }

    Ok(())
}

fn _has_any_rust_files(paths: &[PathBuf]) -> bool {
    let rs = OsStr::new("rs");

    paths
        .iter()
        .any(|path| matches!(path.extension(), Some(ext) if ext == rs))
}
