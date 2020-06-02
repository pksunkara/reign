use reign::router::{
    anyhow::Error as AnyError,
    hyper::{http::Error as HttpError, Body, Response as Res, StatusCode},
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
    fn respond(self) -> Result<Res<Body>, HttpError> {
        match self {
            Reign(e) => e.respond(),
            _ => Res::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty()),
        }
    }
}
