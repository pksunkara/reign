#![feature(proc_macro_hygiene)]

use reign::{
    prelude::*,
    router::middleware::{HeadersDefault, Runtime},
};
use serde_json::{from_str, to_string, Value};
use warp::Filter;

async fn post() -> Result<impl warp::Reply, std::convert::Infallible> {
    Ok("post")
}

async fn put() -> Result<impl warp::Reply, std::convert::Infallible> {
    Ok("put")
}

async fn patch() -> Result<impl warp::Reply, std::convert::Infallible> {
    Ok("patch")
}

async fn delete() -> Result<impl warp::Reply, std::convert::Infallible> {
    Ok("delete")
}

async fn methods() -> Result<impl warp::Reply, std::convert::Infallible> {
    Ok("methods")
}

async fn server() {

    let post = warp::path("post").and(warp::post()).and_then(post);
    let put = warp::path("put").and(warp::put()).and_then(put);
    let patch = warp::path("patch").and(warp::patch()).and_then(patch);
    let delete = warp::path("delete").and(warp::delete()).and_then(delete);

    let methods = warp::path("methods").and(warp::post().or(warp::put()).unify()).and_then(methods);

    let app = post.or(put).or(patch).or(delete).or(methods);

    warp::serve(app).run(([127, 0, 0, 1], 8080)).await;
}

#[tokio::main]
async fn main() {
    server().await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use test_examples::router::{test, StatusCode};
    use tokio::{select, time::delay_for};

    #[tokio::test]
    async fn test_server() {
        let client = async {
            delay_for(Duration::from_millis(100)).await;
            test(StatusCode::METHOD_NOT_ALLOWED).await
        };

        select! {
            _ =  server() => {}
            _ = client => {}
        }
    }
}
