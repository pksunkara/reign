#![doc(html_logo_url = "https://reign.rs/images/media/reign.png")]
#![doc = include_str!("../README.md")]

#[doc(hidden)]
pub use once_cell;

mod boot;
mod config;
mod env;
mod plugin;

pub use boot::Reign;
pub use config::Config;
