use crate::utils::{
    term::{GREEN_BOLD, TERM_ERR},
    Result, INTERNAL_ERR,
};
use handlebars::Handlebars;
use serde::Serialize;
use std::{
    fs::{create_dir_all, File},
    path::PathBuf,
};

pub trait ToRender {
    fn to_render<T>(&self, template: &str, data: &T, project: &PathBuf, path: &[&str]) -> Result
    where
        T: Serialize;
}

impl ToRender for Handlebars<'_> {
    fn to_render<T>(&self, template: &str, data: &T, project: &PathBuf, path: &[&str]) -> Result
    where
        T: Serialize,
    {
        let path_in_project = path.iter().fold(PathBuf::new(), |p, x| p.join(x));
        let full_path = project.join(path_in_project.clone());

        create_dir_all(full_path.clone().parent().expect(INTERNAL_ERR))?;

        TERM_ERR.write_line(&format!(
            "    {} {}",
            GREEN_BOLD.apply_to("create"),
            path_in_project.to_string_lossy()
        ))?;

        Ok(self.render_template_to_write(template, data, File::create(full_path)?)?)
    }
}
