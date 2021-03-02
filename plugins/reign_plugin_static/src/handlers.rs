// Quite a bit of this code is inspired from https://github.com/stephank/hyper-staticfile
// Had to implement my own thing because it does not support some features

//! Contains the handlers

use crate::stream::Stream;

use chrono::{offset::Local, DateTime, SubsecRound, Utc};
use mime_guess::from_path;
use reign_plugin::reign_router::{
    futures::FutureExt,
    hyper::{
        header::{
            HeaderValue, CACHE_CONTROL, CONTENT_LENGTH, CONTENT_TYPE, ETAG, IF_MODIFIED_SINCE,
            IF_NONE_MATCH, LAST_MODIFIED, LOCATION,
        },
        http::{response::Builder, Error as HttpError},
        Body, Method, Response, StatusCode,
    },
    Error, HandleFuture, Request,
};
use tokio::fs::File;

use std::{
    fs::Metadata,
    path::PathBuf,
    time::{Duration, UNIX_EPOCH},
};

const MIN_VALID_MTIME: Duration = Duration::from_secs(2);

pub(crate) fn to_dir(from: Vec<String>, cache: u32) -> impl Fn(&mut Request) -> HandleFuture {
    move |req: &mut Request| {
        let mut path = from.iter().fold(PathBuf::new(), |p, x| p.join(x));

        async move {
            let param = req.param_opt_glob::<String>("path")?;

            if let Some(param) = param {
                for part in param {
                    if part == ".." {
                        return not_found();
                    }

                    path.push(part);
                }
            }

            let path = if let Ok(path) = path.canonicalize() {
                path
            } else {
                return not_found();
            };

            let file = File::open(&path).await?;
            let metadata = file.metadata().await?;

            // If path is directory, redirect to `index.html`
            if metadata.is_dir() {
                return Ok(Response::builder()
                    .status(StatusCode::MOVED_PERMANENTLY)
                    .header(
                        LOCATION,
                        format!("{}/index.html", req.uri().path().trim_end_matches("/")),
                    )
                    .body(Body::empty())?);
            }

            let mime_type =
                HeaderValue::from_str(from_path(&path).first_or_octet_stream().as_ref())
                    .map_err(HttpError::from)?;

            let (modified, etag) = modified_and_etag(&metadata);
            let mut response = build_response(cache, &etag, &modified);

            // Check if we need to send 304
            if let Some(true) = respond_not_modified(
                req.headers().get(IF_NONE_MATCH),
                req.headers().get(IF_MODIFIED_SINCE),
                etag,
                modified,
            ) {
                return Ok(response
                    .status(StatusCode::NOT_MODIFIED)
                    .body(Body::empty())?);
            }

            // Add `Content-Length` and `Content-Type` header
            response = response
                .header(CONTENT_LENGTH, format!("{}", metadata.len()))
                .header(CONTENT_TYPE, mime_type);

            // Build body
            let response = response.body(if *req.method() == Method::HEAD {
                Body::empty()
            } else {
                Body::wrap_stream(Stream::new(file))
            })?;

            Ok(response)
        }
        .boxed()
    }
}

fn not_found() -> Result<Response<Body>, Error> {
    Err(Error::Status(StatusCode::NOT_FOUND))
}

fn modified_and_etag(metadata: &Metadata) -> (Option<DateTime<Local>>, Option<String>) {
    let modified = metadata
        .modified()
        .ok()
        .filter(|v| {
            v.duration_since(UNIX_EPOCH)
                .ok()
                .filter(|v| v >= &MIN_VALID_MTIME)
                .is_some()
        })
        .map(Into::<DateTime<Local>>::into);

    let mut etag = None;

    if let Some(modified) = modified {
        etag = Some(format!(
            "W/\"{0:x}-{1:x}.{2:x}\"",
            metadata.len(),
            modified.timestamp(),
            modified.timestamp_subsec_nanos()
        ));
    }

    (modified, etag)
}

fn build_response(
    cache: u32,
    etag: &Option<String>,
    modified: &Option<DateTime<Local>>,
) -> Builder {
    let mut response = Response::builder();

    // Add `Cache-Control` header
    if cache != 0 {
        response = response.header(CACHE_CONTROL, format!("public, max-age={}", cache));
    }

    // Add `Last-Modified` header
    if let Some(modified) = &modified {
        response = response.header(
            LAST_MODIFIED,
            format!(
                "{} GMT",
                modified.with_timezone(&Utc).format("%a, %d %b %Y %H:%M:%S")
            ),
        )
    }

    // Add `Etag` header
    if let Some(etag) = &etag {
        response = response.header(ETAG, etag);
    }

    response
}

fn respond_not_modified(
    if_none_match: Option<&HeaderValue>,
    if_modified_since: Option<&HeaderValue>,
    etag: Option<String>,
    modified: Option<DateTime<Local>>,
) -> Option<bool> {
    let modified = modified?;
    let etag = etag?;

    // Check `If-None-Match` matches any of the etags
    if let Some(etags) = if_none_match.and_then(|v| v.to_str().ok()) {
        return Some(etags.split(',').any(|x| x.trim() == etag));
    }

    // Check `If-Modified-Since` is newer than actual modified
    if let Some(timestamp) = if_modified_since
        .and_then(|v| v.to_str().ok())
        .and_then(|v| DateTime::parse_from_rfc2822(v).ok())
        .map(|v| v.with_timezone(&Local))
    {
        return Some(modified.trunc_subsecs(0) <= timestamp.trunc_subsecs(0));
    }

    Some(false)
}
