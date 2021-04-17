use reign::router::{
    hyper::{http::Error as HttpError, Body, Response as Res, StatusCode},
    Error as RouterError, Response,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Serde(#[from] serde_json::Error),
    #[error("{0}")]
    Router(#[from] RouterError),
    #[error("{0}")]
    Any(#[from] anyhow::Error),
}

impl Response for Error {
    fn respond(self) -> Result<Res<Body>, HttpError> {
        match self {
            Self::Serde(_) => Res::builder()
                .status(StatusCode::UNPROCESSABLE_ENTITY)
                .body(Body::empty()),
            _ => Res::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty()),
        }
    }
}
