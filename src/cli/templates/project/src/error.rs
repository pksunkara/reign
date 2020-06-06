use reign::router::{
    anyhow::Error as AnyError,
    hyper::{http::Error as HttpError, Body, Response as HyperResponse, StatusCode},
    Error as ReignError, Response,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Http(#[from] HttpError),
    #[error(transparent)]
    Reign(#[from] ReignError),
    #[error(transparent)]
    Any(#[from] AnyError),
}

impl Response for Error {
    fn respond(self) -> Result<HyperResponse<Body>, HttpError> {
        match self {
            Self::Reign(e) => e.respond(),
            _ => HyperResponse::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty()),
        }
    }
}
