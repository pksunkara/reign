use crate::error::Error;
use reign::prelude::*;

#[action]
pub async fn home(_req: &mut Request) -> Result<impl Response, Error> {
    Ok(render!(pages::home)?)
}
