#![doc(html_logo_url = "https://reign.rs/images/media/reign.png")]
#![doc(html_root_url = "https://docs.rs/reign_model/0.2.1")]
#![doc = include_str!("../README.md")]

pub use diesel;
pub use tokio_diesel;

mod connection;
mod error;
#[cfg(feature = "plugin")]
mod plugin;

pub use connection::Database;
pub use error::Error;
