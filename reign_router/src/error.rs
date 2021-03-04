use crate::{
    hyper::{
        http::{header::ToStrError as HttpToStrError, Error as HttpError},
        Body, Error as HyperError, Response as HyperResponse, StatusCode,
    },
    Response,
};

use thiserror::Error;
use tokio::io::Error as TokioIoError;

use std::str::Utf8Error;

/// Error returned by [`Request`](./struct.Request.html) when trying to access params
#[derive(Error, Debug)]
pub enum ParamError {
    #[error("required param `{0}` not found")]
    RequiredParamNotFound(String),
    #[error("required glob param `{0}` not found")]
    RequiredGlobParamNotFound(String),
    #[error("unable to convert param `{0}` from string")]
    UnableToConvertParam(String),
}

/// Main error that can be used by endpoint handlers
///
/// Implements [`Response`](./trait.Response.html) so that this can be converted into
/// a valid server response.
#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Param(#[from] ParamError),
    #[error(transparent)]
    TokioIo(#[from] TokioIoError),
    #[error(transparent)]
    Utf8(#[from] Utf8Error),
    #[error(transparent)]
    Hyper(#[from] HyperError),
    #[error(transparent)]
    Http(#[from] HttpError),
    #[error(transparent)]
    HeaderStr(#[from] HttpToStrError),
    #[error("status {0}")]
    Status(StatusCode),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl Response for Error {
    fn respond(self) -> Result<HyperResponse<Body>, HttpError> {
        match self {
            Self::Param(_) | Self::TokioIo(_) => HyperResponse::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty()),
            Self::Hyper(_) | Self::Utf8(_) => HyperResponse::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::empty()),
            Self::Status(code) => HyperResponse::builder().status(code).body(Body::empty()),
            _ => HyperResponse::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty()),
        }
    }
}
