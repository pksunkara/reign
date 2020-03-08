use gotham::hyper::{Body, Response, StatusCode};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Serde(#[from] serde_json::Error),
    #[error("{0}")]
    Any(#[from] anyhow::Error),
}

impl Error {
    pub fn respond(&self) -> Response<Body> {
        match self {
            Self::Serde(_) => Response::builder()
                .status(StatusCode::UNPROCESSABLE_ENTITY)
                .body(Body::empty())
                .unwrap(),
            _ => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty())
                .unwrap(),
        }
    }
}
