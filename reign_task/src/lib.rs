#![cfg_attr(feature = "doc", feature(external_doc))]
#![doc(html_logo_url = "https://reign.rs/images/media/reign.png")]
#![doc(html_root_url = "https://docs.rs/reign_task/0.2.1")]
#![cfg_attr(feature = "doc", doc(include = "../README.md"))]

#[doc(hidden)]
pub mod error;
mod task;
mod tasks;
#[cfg(feature = "templating")]
mod templating;

pub use error::Error;
pub use task::Task;
pub use tasks::Tasks;
#[cfg(feature = "templating")]
pub use templating::Template;

pub use oclif;
#[cfg(feature = "templating")]
pub use serde_json;
