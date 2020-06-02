use crate::{Error, HandleFuture, Request};
use futures::FutureExt;
use hyper::{
    header::{HeaderValue, CONTENT_TYPE, IF_MODIFIED_SINCE},
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

pub fn to_dir<'a>(base: &'a str, cache: Option<u32>) -> impl Fn(&mut Request) -> HandleFuture + 'a {
    move |req: &mut Request| {
        let mut path = PathBuf::from(base);

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
                HeaderValue::from_str(from_path(&path).first_or_octet_stream().as_ref())?;

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
