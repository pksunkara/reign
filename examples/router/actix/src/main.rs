#![feature(proc_macro_hygiene)]

use actix_web::HttpResponse;
use reign::{
    prelude::*,
    router::middleware::{HeadersDefault, Runtime},
};
use serde_json::{from_str, to_string, Value};

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
    Ok(HttpResponse::Ok().body("response"))
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
fn param() {
    Ok("param")
}

#[action]
fn param_typed() {
    Ok("param_typed")
}

#[action]
fn param_regex() {
    Ok("param_regex")
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

    // TODO:(router) make path optional here
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

        // get!("/param/{foo}", param);
        // get!("/param/{foo:u16}", param_typed);
        // get!("/param/{foo:[a-f]{6}}/{bar:\\d+}", param_regex);
        // TODO:(router) param_optional

        // scope!("/scope-static", {
        //     get!("/", scope_static);
        // });

        // scope!("/pipe", [timer], {
        //     get!("/", pipe);
        // });

        // scope!("/pipe-empty", [], {
        //     get!("/", pipe_empty);
        // });

        // TODO:(router) any
        // TODO:(router) 301, 302
    });

    // scope!("/api", [common, api], {
    //     get!("/", api);
    // });
);

async fn server() {
    router("127.0.0.1:8080").await.unwrap();
}

#[actix_rt::main]
async fn main() {
    server().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_rt::{spawn, time::delay_for};
    use std::time::Duration;
    use test_examples::router::{test, StatusCode};

    #[actix_rt::test]
    async fn test_server() {
        spawn(server());

        let client = async {
            delay_for(Duration::from_millis(100)).await;
            test(StatusCode::NOT_FOUND).await
        };

        client.await;
    }
}
