use reign::router::{
    anyhow::Error as AnyError,
    hyper::{http::Error as HttpError, Body, Response as HyperResponse, StatusCode},
    Error as RouterError, Response,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Http(#[from] HttpError),
    #[error(transparent)]
    Router(#[from] RouterError),
    #[error(transparent)]
    Any(#[from] AnyError),
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
