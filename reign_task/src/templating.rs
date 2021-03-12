use crate::{
    error::INTERNAL_ERR,
    serde_json::{json, Value},
    term::{GREEN_BOLD, TERM_ERR},
    Error,
};

use handlebars::Handlebars;

use std::{
    fs::{create_dir_all, File},
    path::PathBuf,
};

pub struct Template<'a> {
    app_root: &'a PathBuf,
    files: Vec<(PathBuf, &'a str, Value)>,
}

impl<'a> Template<'a> {
    pub fn new(app_root: &'a PathBuf) -> Self {
        Self {
            app_root,
            files: vec![],
        }
    }

    pub fn render(mut self, path: &'a [&'a str], content: &'a str, data: Value) -> Self {
        let path_from_root = path.iter().fold(PathBuf::new(), |p, x| p.join(x));
        self.files.push((path_from_root, content, data));
        self
    }

    pub fn copy(self, path: &'a [&'a str], content: &'a str) -> Self {
        self.render(path, content, json!({}))
    }

    pub fn run(&self) -> Result<(), Error> {
        let handlebars = Handlebars::new();

        for (path, content, data) in &self.files {
            let full_path = self.app_root.join(path.clone());

            create_dir_all(full_path.parent().expect(INTERNAL_ERR))?;

            TERM_ERR.write_line(&format!(
                "    {} {}",
                GREEN_BOLD.apply_to("create"),
                path.to_string_lossy()
            ))?;

            handlebars.render_template_to_write(*content, data, File::create(full_path)?)?;
        }

        Ok(())
    }
}
