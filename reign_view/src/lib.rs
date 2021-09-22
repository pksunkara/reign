#![doc(html_logo_url = "https://reign.rs/images/media/reign.png")]
#![doc(html_root_url = "https://docs.rs/reign_view/0.2.1")]
#![doc = include_str!("../README.md")]

#[doc(hidden)]
pub use maplit;

#[doc(hidden)]
pub mod common;
#[doc(hidden)]
pub mod parse;
mod slots;

#[doc(hidden)]
pub use slots::{slot_render, Slots};

pub(crate) const INTERNAL_ERR: &str =
    "Internal error on reign_view. Please create an issue on https://github.com/pksunkara/reign";
