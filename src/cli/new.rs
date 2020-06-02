use crate::utils::Result;
use clap::Clap;
use handlebars::Handlebars;
use inflector::cases::snakecase::to_snake_case;
use serde::Serialize;
use std::{
    fs::{create_dir_all, File},
    path::PathBuf,
};

#[derive(Debug, Clap)]
pub struct New {
    /// Name of the project
    name: String,
}

#[derive(Serialize)]
pub struct Project {
    name: String,
    reign_version: String,
}

impl New {
    pub fn run(&self) -> Result {
        let mut handlebars = Handlebars::new();

        // TODO:(cli) Allow option to merge
        let project = PathBuf::from(&self.name);
        let src = project.join("src");
        let controllers = src.join("controllers");
        let models = src.join("models");

        create_dir_all(&models)?;
        create_dir_all(&controllers)?;

        handlebars.render_template_to_write(
            include_str!("templates/project/Cargo.toml"),
            &Project {
                name: to_snake_case(&self.name),
                reign_version: env!("CARGO_PKG_VERSION").into(),
            },
            File::create(project.join("Cargo.toml"))?,
        )?;

        Ok(())
    }
}
