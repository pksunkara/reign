use crate::{error::Error, models::Article};

use reign::prelude::*;

#[action]
pub async fn list(_req: &mut Request) -> Result<impl Response, Error> {
    let articles = Article::all().await?;

    Ok(render!(articles::list)?)
}

#[action]
pub async fn new(_req: &mut Request) -> Result<impl Response, Error> {
    Ok(render!(articles::new)?)
}

#[action]
pub async fn create(_req: &mut Request) -> Result<impl Response, Error> {
    Ok("Article Create")
}

#[action]
pub async fn show(_req: &mut Request, id: i32) -> Result<impl Response, Error> {
    let article = Article::filter().id(id).one().await?.ok_or_404()?;

    Ok(render!(articles::show)?)
}
