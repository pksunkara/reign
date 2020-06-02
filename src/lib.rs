#![cfg_attr(feature = "doc", feature(external_doc))]
#![doc(html_logo_url = "https://reign.rs/images/media/reign.png")]
#![doc(html_root_url = "https://docs.rs/reign/0.2.1")]
#![cfg_attr(feature = "doc", doc(include = "../README.md"))]

pub mod prelude;

#[cfg(feature = "framework")]
pub use reign_boot::boot;
#[cfg(feature = "router-backend")]
pub use reign_router as router;
#[cfg(feature = "view")]
pub use reign_view as view;
