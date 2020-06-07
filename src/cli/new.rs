use crate::{
    templates::project,
    utils::{render::ToRender, Result},
};
use clap::Clap;
use handlebars::Handlebars;
use inflector::cases::{snakecase::to_snake_case, titlecase::to_title_case};
use serde::Serialize;
use serde_json::json;
use std::path::PathBuf;

#[derive(Debug, Clap)]
pub struct New {
    /// Name of the project
    name: String,
}

#[derive(Serialize)]
pub struct Name {
    name: String,
}

#[derive(Serialize)]
pub struct Project {
    name: String,
    reign_version: String,
}

impl New {
    pub fn run(&self) -> Result {
        let handlebars = Handlebars::new();

        // TODO:(cli) Allow option to merge
        let project = PathBuf::from(&self.name);

        let name = Name {
            name: to_title_case(&self.name),
        };

        handlebars.to_render(
            project::CARGO_TOML,
            &Project {
                name: to_snake_case(&self.name),
                reign_version: env!("CARGO_PKG_VERSION").into(),
            },
            &project,
            &["Cargo.toml"],
        )?;
        handlebars.to_render(project::README, &name, &project, &["README.md"])?;
        handlebars.to_render(
            project::ENV,
            &Name {
                name: to_snake_case(&self.name),
            },
            &project,
            &[".env"],
        )?;
        handlebars.to_render(project::GITIGNORE, &json!({}), &project, &[".gitignore"])?;

        handlebars.to_render(project::MAIN, &json!({}), &project, &["src", "main.rs"])?;
        handlebars.to_render(project::ERROR, &json!({}), &project, &["src", "error.rs"])?;
        handlebars.to_render(project::CONFIG, &json!({}), &project, &["src", "config.rs"])?;
        handlebars.to_render(project::ROUTES, &json!({}), &project, &["src", "routes.rs"])?;

        handlebars.to_render(
            project::MODELS,
            &json!({}),
            &project,
            &["src", "models", "mod.rs"],
        )?;

        handlebars.to_render(
            project::CONTROLLERS,
            &json!({}),
            &project,
            &["src", "controllers", "mod.rs"],
        )?;
        handlebars.to_render(
            project::PAGES_CONTROLLER,
            &json!({}),
            &project,
            &["src", "controllers", "pages.rs"],
        )?;

        handlebars.to_render(
            project::LAYOUT,
            &name,
            &project,
            &["src", "views", "layouts", "application.html"],
        )?;
        handlebars.to_render(
            project::VIEW,
            &json!({}),
            &project,
            &["src", "views", "pages", "home.html"],
        )?;

        handlebars.to_render(
            project::CSS,
            &json!({}),
            &project,
            &["src", "assets", "css", "app.css"],
        )?;
        handlebars.to_render(
            project::JS,
            &json!({}),
            &project,
            &["src", "assets", "js", "app.js"],
        )?;

        Ok(())
    }
}
