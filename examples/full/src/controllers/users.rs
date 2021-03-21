use crate::{error::Error, models::User};

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

pub async fn login_submit(req: &mut Request) -> Result<impl Response, Error> {
    reign_plugin_auth::login(req, "", "");

    Ok(redirect("/")?)
}

pub async fn logout(req: &mut Request) -> Result<impl Response, Error> {
    reign_plugin_auth::logout(req);

    Ok(redirect("/")?)
}

pub async fn profile(req: &mut Request) -> Result<impl Response, Error> {
    let user = reign_plugin_auth::user(req);

    if user.is_none() {
        return Ok(redirect("/")?);
    }

    let user = User::filter().id(user.unwrap()).one().await?;

    Ok(format!("{:?}", user).respond()?)
}
