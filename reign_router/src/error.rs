use anyhow::Error as AnyError;
use hyper::{http::Error as HttpError, Error as HyperError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParamError {
    #[error("required param `{0}` not found")]
    RequiredParamNotFound(String),
    #[error("required glob param `{0}` not found")]
    RequiredGlobParamNotFound(String),
}

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Param(#[from] ParamError),
    #[error(transparent)]
    Hyper(#[from] HyperError),
    #[error(transparent)]
    Http(#[from] HttpError),
    #[error(transparent)]
    Other(#[from] AnyError),
}
