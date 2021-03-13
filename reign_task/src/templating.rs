use crate::{
    error::INTERNAL_ERR,
    serde_json::{json, Value},
    Error,
};

use handlebars::Handlebars;
use oclif::term::{ERR_GREEN_BOLD, ERR_YELLOW_BOLD, TERM_ERR};

use std::{
    fs::{create_dir_all, read_to_string, write, File},
    path::PathBuf,
    process::Command,
};

pub struct Template<'a> {
    app_root: &'a PathBuf,
    files: Vec<(PathBuf, &'a str, Value)>,
    edits: Vec<(
        PathBuf,
        Box<dyn Fn(String) -> Result<String, Error> + 'static>,
    )>,
}

impl<'a> Template<'a> {
    pub fn new(app_root: &'a PathBuf) -> Self {
        Self {
            app_root,
            files: vec![],
            edits: vec![],
        }
    }

    fn path(&self, path: &[&str]) -> PathBuf {
        path.iter().fold(PathBuf::new(), |p, x| p.join(x))
    }

    pub fn render(mut self, path: &[&str], content: &'a str, data: Value) -> Self {
        self.files.push((self.path(path), content, data));
        self
    }

    pub fn copy(self, path: &[&str], content: &'a str) -> Self {
        self.render(path, content, json!({}))
    }

    pub fn edit<F>(mut self, path: &[&str], f: F) -> Self
    where
        F: Fn(String) -> Result<String, Error> + 'static,
    {
        self.edits.push((self.path(path), Box::new(f)));
        self
    }

    pub fn run(&self) -> Result<(), Error> {
        let handlebars = Handlebars::new();

        for (path, content, data) in &self.files {
            let full_path = self.app_root.join(path.clone());

            create_dir_all(full_path.parent().expect(INTERNAL_ERR))?;

            TERM_ERR.write_line(&format!(
                "    {} {}",
                ERR_GREEN_BOLD.apply_to("create"),
                path.to_string_lossy(),
            ))?;

            handlebars.render_template_to_write(*content, data, File::create(full_path)?)?;
        }

        for (path, f) in &self.edits {
            let full_path = self.app_root.join(path.clone());

            write(&full_path, f(read_to_string(&full_path)?)?)?;

            TERM_ERR.write_line(&format!(
                "    {} {}",
                ERR_YELLOW_BOLD.apply_to("modify"),
                path.to_string_lossy(),
            ))?;
        }

        Command::new("cargo")
            .args(&["fmt"])
            .status()
            .map_err(|_| Error::Cargo)?;

        Ok(())
    }
}
