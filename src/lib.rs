#![cfg_attr(feature = "build-docs", feature(external_doc))]
#![doc(html_root_url = "https://docs.rs/reign/0.1.2")]
#![cfg_attr(feature = "build-docs", doc(include = "../README.md"))]

pub use log;

#[cfg(feature = "framework")]
pub use reign_boot::boot;
pub use reign_derive as prelude;
#[cfg(feature = "router")]
pub use reign_router as router;
pub use reign_view as view;
