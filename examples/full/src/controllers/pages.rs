use crate::error::Error;

use reign::prelude::*;

pub async fn home(_req: &mut Request) -> Result<impl Response, Error> {
    let title = "Home";

    Ok(render!(pages::home)?)
}
