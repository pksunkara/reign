mod error;
pub mod render;
pub mod term;

pub use error::Error;

pub type Result<T = ()> = std::result::Result<T, Error>;

pub const INTERNAL_ERR: &str =
    "Internal error on reign_cli. Please create an issue on https://github.com/pksunkara/reign";