#![cfg_attr(feature = "doc", feature(external_doc))]
#![doc(html_logo_url = "https://reign.rs/images/media/reign.png")]
#![doc(html_root_url = "https://docs.rs/reign_boot/0.2.1")]
#![cfg_attr(feature = "doc", doc(include = "../README.md"))]

pub use once_cell;

use env_logger::{from_env as logger_from_env, Env};

mod config;
mod env;

pub use config::{Config, ConfigLoader};

// TODO: CLI tasks with feature
pub fn boot() -> ConfigLoader {
    env::load_env_files();

    // TODO:(log) Allow custom loggers by adding an option to exclude this call
    logger_from_env(Env::default().default_filter_or("info"))
        .format_timestamp(None)
        .init();

    ConfigLoader {}
}
