use crate::{
    server::{view::build, write::write_file},
    utils::{
        self,
        term::{RED_BOLD, TERM_ERR},
        INTERNAL_ERR,
    },
};
use clap::Clap;
use notify::{
    event::{CreateKind, DataChange, EventKind, ModifyKind, RemoveKind},
    Error, Event, RecommendedWatcher, RecursiveMode, Watcher,
};
use regex::Regex;
use reign_view::common::{tokenize_view, FILE_REGEX, FOLDER_REGEX};
use std::{
    path::{Path, PathBuf},
    process::Command,
};

mod view;
mod write;

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

        build(&views)?;

        // TODO:(cli) Maintain dep graph to reload views that depend on views and build
        // just those views instead of rebuilding everything. Also for views that changed
        // maintain manifest such that manifest is updated only if idents changed

        let _view_watcher = self.view_watcher(views)?;

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

    // fn build_watcher() -> utils::Result<RecommendedWatcher> {
    //     let mut watcher = Watcher::new_immediate(|res: Result<Event, Error>| match res {
    //         Ok(event) => {
    //             println!("{:#?}", event);
    //         }
    //         Err(e) => println!("watch error: {:?}", e),
    //     });

    //     watcher.watch(&views, RecursiveMode::Recursive)?;
    //     Ok(watcher)
    // }

    fn view_watcher(&self, views: PathBuf) -> utils::Result<RecommendedWatcher> {
        let full_path = views.canonicalize()?;

        let mut watcher: RecommendedWatcher =
            Watcher::new_immediate(move |res: Result<Event, Error>| match res {
                Ok(event) => match event.kind {
                    EventKind::Modify(ModifyKind::Data(DataChange::Content)) => {
                        println!("changed content {:?}", event);

                        for path in &event.paths {
                            if let Some((last, view)) = path
                                .strip_prefix(&full_path)
                                .expect(INTERNAL_ERR)
                                .components()
                                .map(|c| c.as_os_str().to_string_lossy())
                                .collect::<Vec<_>>()
                                .split_last()
                            {
                                if FILE_REGEX.is_match(&last)
                                    && view.iter().all(|x| FOLDER_REGEX.is_match(&x))
                                {
                                    let file_base_name = last.trim_end_matches(".html");
                                    let key = format!("{}:{}", view.join(":"), file_base_name);

                                    println!("{:#?}", key);

                                    // TODO: Reload everything

                                    let (tokens, vars) = tokenize_view(path, file_base_name);

                                    let vars = vars
                                        .into_iter()
                                        .map(|(i, b)| (i.to_string(), b))
                                        .collect::<Vec<_>>();

                                    let new_path = path
                                        .parent()
                                        .expect(INTERNAL_ERR)
                                        .join(format!("{}.rs", file_base_name));

                                    write_file(&new_path, tokens.to_string()).unwrap();

                                    println!("{:#?}", vars);
                                }
                            }
                        }
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
        Ok(watcher)
    }
}

fn check_regex(paths: &[PathBuf], regex: &Regex) -> bool {
    paths
        .iter()
        .any(|x| regex.is_match(&x.file_name().expect(INTERNAL_ERR).to_string_lossy()))
}
