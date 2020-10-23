#![cfg_attr(feature = "doc", feature(external_doc))]
#![doc(html_logo_url = "https://reign.rs/images/media/reign.png")]
#![doc(html_root_url = "https://docs.rs/reign_boot/0.2.1")]
#![cfg_attr(feature = "doc", doc(include = "../README.md"))]

pub use once_cell;

use env_logger::{from_env as logger_from_env, Env};
use envy::{from_env, prefixed};
use once_cell::sync::OnceCell;
use serde::Deserialize;

use std::fmt::Debug;

mod env;

pub fn boot() -> Config {
    env::load_env_files();

    // TODO:(log) Allow custom loggers by adding an option to exclude this call
    logger_from_env(Env::default().default_filter_or("info"))
        .format_timestamp(None)
        .init();

    Config {}
}

pub struct Config {}

impl Config {
    pub fn load<T>(&self, cell: &OnceCell<T>) -> &Self
    where
        T: for<'de> Deserialize<'de> + Debug,
    {
        cell.set(from_env::<T>().unwrap()).unwrap();
        self
    }

    pub fn prefixed<T>(&self, cell: &OnceCell<T>, prefix: &str) -> &Self
    where
        T: for<'de> Deserialize<'de> + Debug,
    {
        cell.set(prefixed(prefix).from_env::<T>().unwrap()).unwrap();
        self
    }
}
