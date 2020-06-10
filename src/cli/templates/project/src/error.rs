use reign::{
    model::{tokio_diesel::AsyncError as DieselError, Error as ModelError},
    router::{
        hyper::{http::Error as HttpError, Body, Response as HyperResponse, StatusCode},
        Error as RouterError, Response,
    },
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Diesel(#[from] DieselError),
    #[error(transparent)]
    Model(#[from] ModelError),
    #[error(transparent)]
    Http(#[from] HttpError),
    #[error(transparent)]
    Router(#[from] RouterError),
    #[error(transparent)]
    Any(#[from] anyhow::Error),
}

impl Response for Error {
    fn respond(self) -> Result<HyperResponse<Body>, HttpError> {
        match self {
            Self::Router(e) => e.respond(),
            _ => HyperResponse::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty()),
        }
    }
}
