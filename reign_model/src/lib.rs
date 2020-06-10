#![cfg_attr(feature = "doc", feature(external_doc))]
#![doc(html_logo_url = "https://reign.rs/images/media/reign.png")]
#![doc(html_root_url = "https://docs.rs/reign_model/0.2.1")]
#![cfg_attr(feature = "doc", doc(include = "../README.md"))]

pub use diesel;
pub use tokio_diesel;

mod error;

#[doc(hidden)]
pub mod query;

pub use error::Error;

// pub(crate) const INTERNAL_ERR: &str =
//     "Internal error on reign_model. Please create an issue on https://github.com/pksunkara/reign";
