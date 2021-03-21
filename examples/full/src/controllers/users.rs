use crate::error::Error;

use reign::prelude::*;
use serde::Deserialize;
use serde_urlencoded::from_bytes;

#[derive(Deserialize, Debug)]
struct Register {
    username: String,
    password: String,
}

pub async fn register(_req: &mut Request) -> Result<impl Response, Error> {
    Ok(render!(users::register)?)
}

pub async fn register_submit(req: &mut Request) -> Result<impl Response, Error> {
    println!("{:#?}", req.body().await?.map(|v| from_bytes::<Register>(&v)));
    Ok("submitted")
}

pub async fn login(_req: &mut Request) -> Result<impl Response, Error> {
    Ok("login")
}

pub async fn logout(_req: &mut Request) -> Result<impl Response, Error> {
    Ok("logout")
}
