use crate::error::Error;
use reign::{
    prelude::*,
    router::{Request, Response}
};

#[action]
pub async fn home(_req: &mut Request) -> Result<impl Response, Error> {
    Ok(render!(pages::home)?)
}
