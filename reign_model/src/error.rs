use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Diesel(#[from] tokio_diesel::AsyncError),
}
