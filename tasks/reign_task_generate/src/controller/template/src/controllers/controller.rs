use crate::error::Error;

use reign::prelude::*;{{#each actions}}

pub async fn {{this}}(_req: &mut Request) -> Result<impl Response, Error> {
    Ok("{{this}}")
}{{/each}}
