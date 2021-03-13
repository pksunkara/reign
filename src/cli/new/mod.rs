use clap::Clap;
use inflector::cases::{snakecase::to_snake_case, titlecase::to_title_case};
use reign_task::{serde_json::json, Error, Template};

use std::path::PathBuf;

/// Create a new Reign application
#[derive(Debug, Clap)]
pub struct New {
    /// Name of the application
    pub name: String,
}

impl New {
    pub fn run(&self) -> Result<(), Error> {
        let project = PathBuf::from(&self.name);
        let snake_name = to_snake_case(&self.name);

        Template::new(&project)
            .copy(&["README.md"], include_str!("template/README.md"))
            .copy(&[".gitignore"], include_str!("template/.gitignore"))
            .render(
                &["Cargo.toml"],
                include_str!("template/Cargo.toml"),
                json!({
                    "name": snake_name,
                    "reign_version": env!("CARGO_PKG_VERSION").to_string(),
                }),
            )
            .render(
                &[".env"],
                include_str!("template/.env"),
                json!({
                    "name": snake_name,
                }),
            )
            .copy(&["src", "main.rs"], include_str!("template/src/main.rs"))
            .copy(&["src", "error.rs"], include_str!("template/src/error.rs"))
            .copy(
                &["src", "config.rs"],
                include_str!("template/src/config.rs"),
            )
            .copy(
                &["src", "routes.rs"],
                include_str!("template/src/routes.rs"),
            )
            .copy(
                &["src", "schema.rs"],
                include_str!("template/src/schema.rs"),
            )
            .copy(
                &["src", "controllers", "mod.rs"],
                include_str!("template/src/controllers/mod.rs"),
            )
            .copy(
                &["src", "controllers", "pages.rs"],
                include_str!("template/src/controllers/pages.rs"),
            )
            .copy(
                &["src", "models", "mod.rs"],
                include_str!("template/src/models/mod.rs"),
            )
            .render(
                &["src", "views", "layouts", "application.html"],
                include_str!("template/src/views/layouts/application.html"),
                json!({
                    "name": to_title_case(&self.name),
                }),
            )
            .copy(
                &["src", "views", "pages", "home.html"],
                include_str!("template/src/views/pages/home.html"),
            )
            .copy(
                &["src", "assets", "css", "app.css"],
                include_str!("template/src/assets/css/app.css"),
            )
            .copy(
                &["src", "assets", "js", "app.js"],
                include_str!("template/src/assets/js/app.js"),
            )
            .render(
                &["xtask", "Cargo.toml"],
                include_str!("template/xtask/Cargo.toml"),
                json!({
                    "reign_version": env!("CARGO_PKG_VERSION").to_string(),
                }),
            )
            .render(
                &["xtask", "src", "main.rs"],
                include_str!("template/xtask/src/main.rs"),
                json!({
                    "name": snake_name,
                }),
            )
            .run()
    }
}
