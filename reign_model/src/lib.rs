#![cfg_attr(feature = "doc", feature(external_doc))]
#![doc(html_logo_url = "https://reign.rs/images/media/reign.png")]
#![doc(html_root_url = "https://docs.rs/reign_model/0.2.1")]
#![cfg_attr(feature = "doc", doc(include = "../README.md"))]

pub use diesel;
pub use tokio_diesel;

mod error;
#[cfg(feature = "framework")]
mod plugin;

#[doc(hidden)]
pub mod query;

pub use error::Error;
#[cfg(feature = "framework")]
pub use plugin::DatabasePlugin;
