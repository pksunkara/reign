#![feature(proc_macro_hygiene)]

use reign::{
    prelude::*,
    router::{
        middleware::{HeadersDefault, Runtime},
        serve, Response, Router,
    },
};
use serde_json::{from_str, to_string, Value};
use tide::{http::StatusCode};

mod errors;

use errors::Error;

#[action]
async fn str_() -> Result<impl Response, Error> {
    Ok("str")
}

#[action]
async fn string() -> Result<impl Response, Error> {
    Ok("string".to_string())
}

#[action]
async fn response() -> Result<impl Response, Error> {
    Ok(tide::Response::new(StatusCode::Ok).body_string("response".to_string()))
}

#[action]
async fn error() -> Result<impl Response, Error> {
    let value = from_str::<Value>("{name}")?;
    Ok(to_string(&value)?)
}

#[action]
async fn post() -> Result<impl Response, Error> {
    Ok("post")
}

#[action]
async fn put() -> Result<impl Response, Error> {
    Ok("put")
}

#[action]
async fn patch() -> Result<impl Response, Error> {
    Ok("patch")
}

#[action]
async fn delete() -> Result<impl Response, Error> {
    Ok("delete")
}

#[action]
async fn multi_methods() -> Result<impl Response, Error> {
    Ok("multi_methods")
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

async fn multiple_param_glob(req: tide::Request<()>) -> tide::Result<tide::Response> {
    Ok(tide::Response::from(format!(
        "multiple_param_glob {} {}",
        req.param::<String>("foo").unwrap(),
        req.param::<String>("bar").unwrap()
    )))
}

#[action]
async fn scope_static_b() -> Result<impl Response, Error> {
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
async fn double_slashes() -> Result<impl Response, Error> {
    Ok("double_slashes")
}

#[action]
async fn sibling_scope_higher() -> Result<impl Response, Error> {
    Ok("sibling_scope_higher")
}

#[action]
async fn sibling_scope_common_higher() -> Result<impl Response, Error> {
    Ok("sibling_scope_common_higher")
}

#[action]
async fn sibling_scope_common_lower() -> Result<impl Response, Error> {
    Ok("sibling_scope_common_lower")
}

#[action]
async fn sibling_scope_common_c() -> Result<impl Response, Error> {
    Ok("sibling_scope_common_c")
}

#[action]
async fn sibling_scope_lower() -> Result<impl Response, Error> {
    Ok("sibling_scope_lower")
}

#[action]
async fn scope_static() -> Result<impl Response, Error> {
    Ok("scope_static")
}

#[action]
async fn pipe() -> Result<impl Response, Error> {
    Ok("pipe")
}

#[action]
async fn pipe_empty() -> Result<impl Response, Error> {
    Ok("pipe_empty")
}

#[router]
fn router() -> Router {
    pipe!(common, [
        HeadersDefault::empty().add("x-powered-by", "reign"),
    ]);
    pipe!(app, [
        HeadersDefault::empty().add("x-content-type-options", "nosniff"),
    ]);
    pipe!(timer, [
        Runtime::default(),
    ]);
    pipe!(api, [
        HeadersDefault::empty().add("x-version", "1.0"),
        HeadersDefault::empty().add("content-type", "application/json"),
    ]);

    scope!("/", [common, app], {
        to!(get, "str", str_);
        to!(get, "string", string);
        to!(get, "response", response);

        to!(get, "error", error);

        to!(post, "post", post);
        to!(put, "put", put);
        to!(patch, "patch", patch);
        to!(delete, "delete", delete);

        to!([post, put], "multi_methods", multi_methods);

        app.at("param/:id").get(param);

        app.at("param_optional").get(param_optional);
        app.at("param_optional/:id").get(param_optional);

        app.at("param_glob/*id").get(param_glob);

        app.at("param_optional_glob").get(param_optional_glob);
        app.at("param_optional_glob/*id").get(param_optional_glob);

        app.at("param_glob_after/*id/b").get(param_glob_after);

        app.at("param_optional_glob_after/b").get(param_optional_glob_after);
        app.at("param_optional_glob_after/*id/b").get(param_optional_glob_after);

        app.at("multiple_param_glob/*foo/foobar/*bar").get(multiple_param_glob);

        to!(get, "double//slashes", double_slashes);

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
            to!(get, "/", scope_static);
        });

        scope!("/pipe", [timer], {
            to!(get, "/", pipe);
        });

        scope!("/pipe-empty", [], {
            to!(get, "/", pipe_empty);
        });
    });
}

async fn server() {
    serve("127.0.0.1:8080", router()).await.unwrap()
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
