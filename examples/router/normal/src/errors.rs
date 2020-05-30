use reign::router::router::{
    hyper::{http::Error as HttpError, Body, Response as HyperResponse, StatusCode},
    Response,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Serde(#[from] serde_json::Error),
    #[error("{0}")]
    Any(#[from] anyhow::Error),
}

impl Response for Error {
    fn respond(self) -> Result<HyperResponse<Body>, HttpError> {
        match self {
            Self::Serde(_) => HyperResponse::builder()
                .status(StatusCode::UNPROCESSABLE_ENTITY)
                .body(Body::empty()),
            _ => HyperResponse::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty()),
        }
    }
}
