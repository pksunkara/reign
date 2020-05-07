#![feature(proc_macro_hygiene)]

use reign::{
    prelude::*,
    router::middleware::{HeadersDefault, Runtime},
};
use serde_json::{from_str, to_string, Value};
use tide::Response;

mod errors;

#[action]
fn str_() {
    Ok("str")
}

#[action]
fn string() {
    Ok("string".to_string())
}

#[action]
fn response() {
    Ok(Response::new(200).body_string("response".to_string()))
}

#[action]
fn error() {
    let value = from_str::<Value>("{name}")?;
    Ok(to_string(&value)?)
}

#[action]
fn post() {
    Ok("post")
}

#[action]
fn put() {
    Ok("put")
}

#[action]
fn patch() {
    Ok("patch")
}

#[action]
fn delete() {
    Ok("delete")
}

#[action]
fn methods() {
    Ok("methods")
}

#[action]
fn scope_static() {
    Ok("scope_static")
}

#[action]
fn pipe() {
    Ok("pipe")
}

#[action]
fn pipe_empty() {
    Ok("pipe_empty")
}

router!(
    pipelines!(
        common: [
            HeadersDefault::empty().add("x-powered-by", "reign"),
        ],
        app: [
            HeadersDefault::empty().add("x-content-type-options", "nosniff"),
        ],
        timer: [
            Runtime::default(),
        ],
        api: [
            HeadersDefault::empty().add("x-version", "1.0"),
            HeadersDefault::empty().add("content-type", "application/json"),
        ],
    );

    scope!("/", [common, app], {
        get!("/str", str_);
        get!("/string", string);
        get!("/response", response);

        get!("/error", error);

        post!("/post", post);
        put!("/put", put);
        patch!("/patch", patch);
        delete!("/delete", delete);

        methods!([post, put], "/methods", methods);

        scope!("/scope-static", {
            get!("/", scope_static);
        });

        scope!("/pipe", [timer], {
            get!("/", pipe);
        });

        scope!("/pipe-empty", [], {
            get!("/", pipe_empty);
        });
    });
);

async fn server() {
    router("127.0.0.1:8080").await.unwrap();
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
