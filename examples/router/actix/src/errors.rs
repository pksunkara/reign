use actix_web::HttpResponse;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Serde(#[from] serde_json::Error),
    #[error("{0}")]
    Any(#[from] anyhow::Error),
}

impl Error {
    pub fn respond(&self) -> HttpResponse {
        match self {
            Self::Serde(_) => HttpResponse::UnprocessableEntity().finish(),
            _ => HttpResponse::InternalServerError().finish(),
        }
    }
}
