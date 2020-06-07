pub static CARGO_TOML: &str = include_str!("Cargo.toml");
pub static README: &str = include_str!("README.md");
pub static GITIGNORE: &str = include_str!(".gitignore");
pub static ENV: &str = include_str!(".env");

#[cfg(not(windows))]
pub static MAIN: &str = include_str!("src/main.rs");
#[cfg(windows)]
pub static MAIN: &str = include_str!("src\\main.rs");

#[cfg(not(windows))]
pub static ERROR: &str = include_str!("src/error.rs");
#[cfg(windows)]
pub static ERROR: &str = include_str!("src\\error.rs");

#[cfg(not(windows))]
pub static ROUTES: &str = include_str!("src/routes.rs");
#[cfg(windows)]
pub static ROUTES: &str = include_str!("src\\routes.rs");

#[cfg(not(windows))]
pub static CONFIG: &str = include_str!("src/config.rs");
#[cfg(windows)]
pub static CONFIG: &str = include_str!("src\\config.rs");

#[cfg(not(windows))]
pub static CONTROLLERS: &str = include_str!("src/controllers/mod.rs");
#[cfg(windows)]
pub static CONTROLLERS: &str = include_str!("src\\controllers\\mod.rs");

#[cfg(not(windows))]
pub static PAGES_CONTROLLER: &str = include_str!("src/controllers/pages.rs");
#[cfg(windows)]
pub static PAGES_CONTROLLER: &str = include_str!("src\\controllers\\pages.rs");

#[cfg(not(windows))]
pub static MODELS: &str = include_str!("src/models/mod.rs");
#[cfg(windows)]
pub static MODELS: &str = include_str!("src\\models\\mod.rs");

#[cfg(not(windows))]
pub static LAYOUT: &str = include_str!("src/views/layouts/application.html");
#[cfg(windows)]
pub static LAYOUT: &str = include_str!("src\\views\\layouts\\application.html");

#[cfg(not(windows))]
pub static VIEW: &str = include_str!("src/views/pages/home.html");
#[cfg(windows)]
pub static VIEW: &str = include_str!("src\\views\\pages\\home.html");

#[cfg(not(windows))]
pub static CSS: &str = include_str!("src/assets/css/app.css");
#[cfg(windows)]
pub static CSS: &str = include_str!("src\\assets\\css\\app.css");

#[cfg(not(windows))]
pub static JS: &str = include_str!("src/assets/js/app.js");
#[cfg(windows)]
pub static JS: &str = include_str!("src\\assets\\js\\app.js");
