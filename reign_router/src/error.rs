use anyhow::Error as AnyError;
use hyper::{
    header::InvalidHeaderValue,
    http::{status::InvalidStatusCode, Error as HttpError},
    Body, Error as HyperError, Response as Res, StatusCode,
};
use thiserror::Error;
use tokio::io::Error as TokioIoError;

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
    InvalidStatusCode(#[from] InvalidStatusCode),
    #[error(transparent)]
    InvalidHeaderValue(#[from] InvalidHeaderValue),
    #[error(transparent)]
    TokioIo(#[from] TokioIoError),
    #[error(transparent)]
    Hyper(#[from] HyperError),
    #[error(transparent)]
    Http(#[from] HttpError),
    #[error(transparent)]
    Other(#[from] AnyError),
}

impl Error {
    pub fn respond(self) -> Result<Res<Body>, HttpError> {
        match self {
            Self::Param(_) | Self::TokioIo(_) => Res::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty()),
            Self::Hyper(_) => Res::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::empty()),
            _ => Res::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty()),
        }
    }
}
