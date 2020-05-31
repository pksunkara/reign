use crate::hyper::http::Error as HttpError;
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
    Http(#[from] HttpError),
}
