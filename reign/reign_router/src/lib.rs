#![cfg_attr(feature = "doc", feature(external_doc))]
#![doc(html_root_url = "https://docs.rs/reign_router/0.1.2")]
#![cfg_attr(feature = "doc", doc(include = "../README.md"))]

pub mod middleware;

#[cfg(feature = "router-actix")]
pub mod actix;
#[cfg(feature = "router-gotham")]
pub mod gotham;
#[cfg(feature = "router-tide")]
pub mod tide;
#[cfg(feature = "router-warp")]
pub mod warp;

pub trait RouterTypeTrait {
    const TYPE: &'static str;
}
