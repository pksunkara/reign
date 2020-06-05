//! Contains some common endpoint handlers

use crate::{Error, HandleFuture, Request};
use futures::FutureExt;
use hyper::{
    header::{HeaderValue, CONTENT_TYPE, IF_MODIFIED_SINCE},
    http::Error as HttpError,
    Body, Response, StatusCode,
};
use hyper_staticfile::FileResponseBuilder;
use mime_guess::from_path;
use std::path::PathBuf;
use tokio::fs::File;

fn not_found() -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::empty())?)
}

/// Route the globbed endpoint to a directory for serving static content
///
/// The endpoint should contain a glob path parameter called `path`.
///
/// # Examples
///
/// ```
/// use reign::{prelude::*, router::{handlers::to_dir, Router}};
///
/// fn router(r: &mut Router) {
///     r.get(p!(path*), to_dir(&["src", "assets"], None));
/// }
/// ```
pub fn to_dir<'a>(
    from: &'a [&'a str],
    cache: Option<u32>,
) -> impl Fn(&mut Request) -> HandleFuture + 'a {
    move |req: &mut Request| {
        let mut path = from.into_iter().fold(PathBuf::new(), |p, x| p.join(x));

        async move {
            for part in req.param_glob("path")? {
                if part == ".." {
                    return not_found();
                }

                path.push(part);
            }

            let path = if let Ok(path) = path.canonicalize() {
                path
            } else {
                return not_found();
            };

            let mime_type =
                HeaderValue::from_str(from_path(&path).first_or_octet_stream().as_ref())
                    .map_err(HttpError::from)?;

            // TODO:(router:file) Compression, IF_NONE_MATCH
            let file = File::open(path).await?;
            let metadata = file.metadata().await?;

            let mut response = FileResponseBuilder::new()
                .request_parts(req.method(), req.headers())
                .cache_headers(cache)
                .if_modified_since_header(req.headers().get(IF_MODIFIED_SINCE))
                .build(file, metadata)?;

            response.headers_mut().insert(CONTENT_TYPE, mime_type);

            Ok(response)
        }
        .boxed()
    }
}
