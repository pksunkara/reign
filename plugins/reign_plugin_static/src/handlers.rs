//! Contains the handlers

use hyper_staticfile::FileResponseBuilder;
use mime_guess::from_path;
use reign_plugin::reign_router::{
    futures::FutureExt,
    hyper::{
        header::{HeaderValue, CONTENT_TYPE, IF_MODIFIED_SINCE},
        http::Error as HttpError,
        Body, Response, StatusCode,
    },
    Error, HandleFuture, Request,
};
use tokio::fs::File;

use std::path::PathBuf;

fn not_found() -> Result<Response<Body>, Error> {
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::empty())?)
}

pub(crate) fn to_dir(from: Vec<String>, cache: Option<u32>) -> impl Fn(&mut Request) -> HandleFuture {
    move |req: &mut Request| {
        let mut path = from.iter().fold(PathBuf::new(), |p, x| p.join(x));

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

            // TODO: plugin:static: Compression, IF_NONE_MATCH
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
