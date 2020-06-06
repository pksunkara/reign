pub static CARGO_TOML: &'static str = include_str!("Cargo.toml");

#[cfg(not(windows))]
pub static MAIN: &'static str = include_str!("src/main.rs");
#[cfg(windows)]
pub static MAIN: &'static str = include_str!("src\\main.rs");

#[cfg(not(windows))]
pub static ERROR: &'static str = include_str!("src/error.rs");
#[cfg(windows)]
pub static ERROR: &'static str = include_str!("src\\error.rs");

#[cfg(not(windows))]
pub static ROUTES: &'static str = include_str!("src/routes.rs");
#[cfg(windows)]
pub static ROUTES: &'static str = include_str!("src\\routes.rs");

#[cfg(not(windows))]
pub static CONFIG: &'static str = include_str!("src/config.rs");
#[cfg(windows)]
pub static CONFIG: &'static str = include_str!("src\\config.rs");

#[cfg(not(windows))]
pub static CONTROLLERS: &'static str = include_str!("src/controllers/mod.rs");
#[cfg(windows)]
pub static CONTROLLERS: &'static str = include_str!("src\\controllers\\mod.rs");

#[cfg(not(windows))]
pub static PAGES_CONTROLLER: &'static str = include_str!("src/controllers/pages.rs");
#[cfg(windows)]
pub static PAGES_CONTROLLER: &'static str = include_str!("src\\controllers\\pages.rs");

#[cfg(not(windows))]
pub static MODELS: &'static str = include_str!("src/models/mod.rs");
#[cfg(windows)]
pub static MODELS: &'static str = include_str!("src\\models\\mod.rs");

#[cfg(not(windows))]
pub static LAYOUT: &'static str = include_str!("src/views/layouts/application.html");
#[cfg(windows)]
pub static LAYOUT: &'static str = include_str!("src\\views\\layouts\\application.html");

#[cfg(not(windows))]
pub static VIEW: &'static str = include_str!("src/views/pages/home.html");
#[cfg(windows)]
pub static VIEW: &'static str = include_str!("src\\views\\pages\\home.html");
