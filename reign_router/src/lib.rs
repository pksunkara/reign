#![doc(html_root_url = "https://docs.rs/reign_router/0.1.2")]

pub mod middleware;

#[cfg(feature = "router-gotham")]
mod redirect;

#[cfg(feature = "router-gotham")]
pub use redirect::redirect;
