#![cfg_attr(feature = "doc", feature(external_doc))]
#![doc(html_logo_url = "https://reign.rs/images/media/reign.png")]
#![doc(html_root_url = "https://docs.rs/reign_boot/0.2.1")]
#![cfg_attr(feature = "doc", doc(include = "../README.md"))]

#[doc(hidden)]
pub use once_cell;

mod boot;
mod config;
mod env;
mod plugin;

pub use boot::Reign;
pub use config::Config;
