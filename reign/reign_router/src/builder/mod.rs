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
