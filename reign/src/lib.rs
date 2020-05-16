#![cfg_attr(feature = "doc", feature(external_doc))]
#![doc(html_root_url = "https://docs.rs/reign/0.1.2")]
#![cfg_attr(feature = "doc", doc(include = "../README.md"))]

pub use log;

#[cfg(feature = "framework")]
pub use reign_boot::boot;
#[cfg(any(feature = "view", feature = "router"))]
pub use reign_derive as prelude;
#[cfg(feature = "router")]
pub use reign_router as router;
#[cfg(feature = "view")]
pub use reign_view as view;
