#![feature(proc_macro_hygiene)]

use reign::{
    prelude::*,
    router::middleware::{HeadersDefault, Runtime},
};
use serde_json::{from_str, to_string, Value};
use tide::{http::StatusCode, Response};

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
    Ok(Response::new(StatusCode::Ok).body_string("response".to_string()))
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

async fn param(req: tide::Request<()>) -> tide::Result<tide::Response> {
    Ok(tide::Response::from(format!(
        "param {}",
        req.param::<String>("id").unwrap()
    )))
}

async fn param_optional(req: tide::Request<()>) -> tide::Result<tide::Response> {
    Ok(tide::Response::from(format!(
        "param_optional {}",
        match req.param::<String>("id") {
            Ok(id) => id,
            Err(_) => "".to_string(),
        }
    )))
}

async fn param_glob(req: tide::Request<()>) -> tide::Result<tide::Response> {
    Ok(tide::Response::from(format!(
        "param_glob {}",
        req.param::<String>("id").unwrap()
    )))
}

async fn param_optional_glob(req: tide::Request<()>) -> tide::Result<tide::Response> {
    Ok(tide::Response::from(format!(
        "param_optional_glob {}",
        match req.param::<String>("id") {
            Ok(id) => id,
            Err(_) => "".to_string(),
        }
    )))
}

async fn param_glob_after(req: tide::Request<()>) -> tide::Result<tide::Response> {
    Ok(tide::Response::from(format!(
        "param_glob_after {}",
        req.param::<String>("id").unwrap()
    )))
}

async fn param_optional_glob_after(req: tide::Request<()>) -> tide::Result<tide::Response> {
    Ok(tide::Response::from(format!(
        "param_optional_glob_after {}",
        match req.param::<String>("id") {
            Ok(id) => id,
            Err(_) => "".to_string(),
        }
    )))
}

#[action]
fn scope_static_b() {
    Ok("scope_static_b")
}

async fn scope_param(req: tide::Request<()>) -> tide::Result<tide::Response> {
    Ok(tide::Response::from(format!(
        "scope_param {}",
        req.param::<String>("id").unwrap()
    )))
}

async fn scope_param_b(req: tide::Request<()>) -> tide::Result<tide::Response> {
    Ok(tide::Response::from(format!(
        "scope_param_b {}",
        req.param::<String>("id").unwrap()
    )))
}

async fn scope_param_optional(req: tide::Request<()>) -> tide::Result<tide::Response> {
    Ok(tide::Response::from(format!(
        "scope_param_optional {}",
        match req.param::<String>("id") {
            Ok(id) => id,
            Err(_) => "".to_string(),
        }
    )))
}

async fn scope_param_optional_b(req: tide::Request<()>) -> tide::Result<tide::Response> {
    Ok(tide::Response::from(format!(
        "scope_param_optional_b {}",
        match req.param::<String>("id") {
            Ok(id) => id,
            Err(_) => "".to_string(),
        }
    )))
}

async fn scope_param_glob(req: tide::Request<()>) -> tide::Result<tide::Response> {
    Ok(tide::Response::from(format!(
        "scope_param_glob {}",
        req.param::<String>("id").unwrap()
    )))
}

async fn scope_param_glob_b(req: tide::Request<()>) -> tide::Result<tide::Response> {
    Ok(tide::Response::from(format!(
        "scope_param_glob_b {}",
        req.param::<String>("id").unwrap()
    )))
}

async fn scope_param_optional_glob(req: tide::Request<()>) -> tide::Result<tide::Response> {
    Ok(tide::Response::from(format!(
        "scope_param_optional_glob {}",
        match req.param::<String>("id") {
            Ok(id) => id,
            Err(_) => "".to_string(),
        }
    )))
}

async fn scope_param_optional_glob_b(req: tide::Request<()>) -> tide::Result<tide::Response> {
    Ok(tide::Response::from(format!(
        "scope_param_optional_glob_b {}",
        match req.param::<String>("id") {
            Ok(id) => id,
            Err(_) => "".to_string(),
        }
    )))
}

async fn scope_param_glob_after(req: tide::Request<()>) -> tide::Result<tide::Response> {
    Ok(tide::Response::from(format!(
        "scope_param_glob_after {}",
        req.param::<String>("id").unwrap()
    )))
}

async fn scope_param_glob_after_b(req: tide::Request<()>) -> tide::Result<tide::Response> {
    Ok(tide::Response::from(format!(
        "scope_param_glob_after_b {}",
        req.param::<String>("id").unwrap()
    )))
}

async fn scope_param_optional_glob_after(req: tide::Request<()>) -> tide::Result<tide::Response> {
    Ok(tide::Response::from(format!(
        "scope_param_optional_glob_after {}",
        match req.param::<String>("id") {
            Ok(id) => id,
            Err(_) => "".to_string(),
        }
    )))
}

async fn scope_param_optional_glob_after_b(req: tide::Request<()>) -> tide::Result<tide::Response> {
    Ok(tide::Response::from(format!(
        "scope_param_optional_glob_after_b {}",
        match req.param::<String>("id") {
            Ok(id) => id,
            Err(_) => "".to_string(),
        }
    )))
}

async fn nested_scope(req: tide::Request<()>) -> tide::Result<tide::Response> {
    Ok(tide::Response::from(format!(
        "nested_scope {}",
        req.param::<String>("id").unwrap()
    )))
}

async fn nested_scope_b(req: tide::Request<()>) -> tide::Result<tide::Response> {
    Ok(tide::Response::from(format!(
        "nested_scope_b {}",
        req.param::<String>("id").unwrap()
    )))
}

#[action]
fn double_slashes() {
    Ok("double_slashes")
}

#[action]
fn sibling_scope_higher() {
    Ok("sibling_scope_higher")
}

#[action]
fn sibling_scope_common_higher() {
    Ok("sibling_scope_common_higher")
}

#[action]
fn sibling_scope_common_lower() {
    Ok("sibling_scope_common_lower")
}

#[action]
fn sibling_scope_common_c() {
    Ok("sibling_scope_common_c")
}

#[action]
fn sibling_scope_lower() {
    Ok("sibling_scope_lower")
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

#[router]
fn router() {
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
        get!("str", str_);
        get!("string", string);
        get!("response", response);

        get!("error", error);

        post!("post", post);
        put!("put", put);
        patch!("patch", patch);
        delete!("delete", delete);

        methods!([post, put], "methods", methods);

        app.at("param/:id").get(param);

        app.at("param_optional").get(param_optional);
        app.at("param_optional/:id").get(param_optional);

        app.at("param_glob/*id").get(param_glob);

        app.at("param_optional_glob").get(param_optional_glob);
        app.at("param_optional_glob/*id").get(param_optional_glob);

        app.at("param_glob_after/*id/b").get(param_glob_after);

        app.at("param_optional_glob_after/b").get(param_optional_glob_after);
        app.at("param_optional_glob_after/*id/b").get(param_optional_glob_after);

        get!("double//slashes", double_slashes);

        app.at("scope_static").nest({
            let mut app = tide::new();

            app.at("").get(scope_static);
            app.at("b").get(scope_static_b);

            app
        });

        app.at("scope_param/:id").nest({
            let mut app = tide::new();

            app.at("b").get(scope_param_b);
            app.at("").get(scope_param);

            app
        });

        app.at("scope_param_optional").nest({
            let mut app = tide::new();

            app.at("b").get(scope_param_optional_b);
            app.at("").get(scope_param_optional);

            app
        });

        app.at("scope_param_optional/:id").nest({
            let mut app = tide::new();

            app.at("b").get(scope_param_optional_b);
            app.at("").get(scope_param_optional);

            app
        });

        app.at("scope_param_glob/*id").nest({
            let mut app = tide::new();

            app.at("b").get(scope_param_glob_b);
            app.at("").get(scope_param_glob);

            app
        });

        app.at("scope_param_optional_glob").nest({
            let mut app = tide::new();

            app.at("b").get(scope_param_optional_glob_b);
            app.at("").get(scope_param_optional_glob);

            app
        });

        app.at("scope_param_optional_glob/*id").nest({
            let mut app = tide::new();

            app.at("b").get(scope_param_optional_glob_b);
            app.at("").get(scope_param_optional_glob);

            app
        });

        app.at("scope_param_glob_after/*id/foo").nest({
            let mut app = tide::new();

            app.at("b").get(scope_param_glob_after_b);
            app.at("").get(scope_param_glob_after);

            app
        });

        app.at("scope_param_optional_glob_after").nest({
            let mut app = tide::new();

            app.at("b").get(scope_param_optional_glob_after_b);
            app.at("").get(scope_param_optional_glob_after);

            app
        });

        app.at("scope_param_optional_glob_after/*id/foo").nest({
            let mut app = tide::new();

            app.at("b").get(scope_param_optional_glob_after_b);
            app.at("").get(scope_param_optional_glob_after);

            app
        });

        app.at("nested_scope/:id").nest({
            let mut app = tide::new();

            app.at("nested_scope_inner").nest({
                let mut app = tide::new();

                app.at("b").get(nested_scope_b);
                app.at("").get(nested_scope);

                app
            });

            app
        });

        app.at("sibling_scope/higher").nest({
            let mut app = tide::new();
            app.at("").get(sibling_scope_higher);
            app
        });
        app.at("sibling_scope").nest({
            let mut app = tide::new();
            app.at("higher").get(sibling_scope_common_higher);
            app.at("lower").get(sibling_scope_common_lower);
            app.at("c").get(sibling_scope_common_c);
            app
        });
        app.at("sibling_scope/lwer").nest({
            let mut app = tide::new();
            app.at("").get(sibling_scope_lower);
            app
        });

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
}

async fn server() {
    router("127.0.0.1:8300").await.unwrap();
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
