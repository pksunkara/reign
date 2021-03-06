use crate::{
    error::INTERNAL_ERR,
    serde_json::{json, Value},
    term::{GREEN_BOLD, TERM_ERR, YELLOW_BOLD},
    Error,
};

use handlebars::Handlebars;

use std::{
    fs::{create_dir_all, File, write, read_to_string},
    path::PathBuf,
};

pub struct Template<'a> {
    app_root: &'a PathBuf,
    files: Vec<(PathBuf, &'a str, Value)>,
    edits: Vec<(PathBuf, Box<dyn Fn(String) -> Result<String, Error> + 'static>)>
}

impl<'a> Template<'a> {
    pub fn new(app_root: &'a PathBuf) -> Self {
        Self {
            app_root,
            files: vec![],
            edits: vec![],
        }
    }

    fn path(&self, path: &'a [&'a str]) -> PathBuf {
        path.iter().fold(PathBuf::new(), |p, x| p.join(x))
    }

    pub fn render(mut self, path: &'a [&'a str], content: &'a str, data: Value) -> Self {
        self.files.push((self.path(path), content, data));
        self
    }

    pub fn copy(self, path: &'a [&'a str], content: &'a str) -> Self {
        self.render(path, content, json!({}))
    }

    pub fn edit<F>(mut self, path: &'a [&'a str], f: F) -> Self
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
                GREEN_BOLD.apply_to("create"),
                path.to_string_lossy(),
            ))?;

            handlebars.render_template_to_write(*content, data, File::create(full_path)?)?;
        }

        for (path, f) in &self.edits {
            let full_path = self.app_root.join(path.clone());

            write(&full_path, f(read_to_string(&full_path)?)?)?;

            TERM_ERR.write_line(&format!(
                "    {} {}",
                YELLOW_BOLD.apply_to("modify"),
                path.to_string_lossy(),
            ))?;
        }

        Ok(())
    }
}
