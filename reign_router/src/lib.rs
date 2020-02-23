#[cfg(feature = "router-gotham")]
mod redirect;

#[cfg(feature = "router-gotham")]
pub use redirect::redirect;
