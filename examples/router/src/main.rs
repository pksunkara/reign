#![feature(proc_macro_hygiene)]

use reign::{
    prelude::*,
    router::{
        middleware::{HeadersDefault, Runtime},
        router::{
            hyper::{self, Method},
            serve, Path, Pipe, Request, Response, Router,
        },
    },
};
use serde_json::{from_str, to_string, Value};

mod errors;

use errors::Error;

#[action]
async fn str_(req: Request) -> Result<impl Response, Error> {
    Ok("str")
}

#[action]
async fn string(req: Request) -> Result<impl Response, Error> {
    Ok("string".to_string())
}

#[action]
async fn response(req: Request) -> Result<impl Response, Error> {
    Ok(hyper::Response::new("response".into()))
}

#[action]
async fn error(req: Request) -> Result<impl Response, Error> {
    let value = from_str::<Value>("{name}")?;
    Ok(hyper::Response::new(to_string(&value)?.into()))
}

#[action]
async fn post(req: Request) -> Result<impl Response, Error> {
    Ok("post")
}

#[action]
async fn put(req: Request) -> Result<impl Response, Error> {
    Ok("put")
}

#[action]
async fn patch(req: Request) -> Result<impl Response, Error> {
    Ok("patch")
}

#[action]
async fn delete(req: Request) -> Result<impl Response, Error> {
    Ok("delete")
}

#[action]
async fn multi_methods(req: Request) -> Result<impl Response, Error> {
    Ok("multi_methods")
}

#[action]
async fn scope_static(req: Request) -> Result<impl Response, Error> {
    Ok("scope_static")
}

#[action]
async fn pipe(req: Request) -> Result<impl Response, Error> {
    Ok("pipe")
}

#[action]
async fn param(req: Request, id: String) -> Result<impl Response, Error> {
    Ok(format!("param {}", id))
}

#[action]
async fn param_opt(req: Request, id: Option<String>) -> Result<impl Response, Error> {
    Ok(format!(
        "param_opt {}",
        match id {
            Some(x) => x,
            None => "".to_string(),
        }
    ))
}

#[action]
async fn param_regex(req: Request, id: String) -> Result<impl Response, Error> {
    Ok(format!("param_regex {}", id))
}

#[action]
async fn param_opt_regex(req: Request, id: Option<String>) -> Result<impl Response, Error> {
    Ok(format!(
        "param_opt_regex {}",
        match id {
            Some(x) => x,
            None => "".to_string(),
        }
    ))
}

#[action]
async fn scope_param(req: Request, id: String) -> Result<impl Response, Error> {
    Ok(format!("scope_param {}", id))
}

#[action]
async fn scope_param_opt(req: Request, id: Option<String>) -> Result<impl Response, Error> {
    Ok(format!(
        "scope_param_opt {}",
        match id {
            Some(x) => x,
            None => "".to_string(),
        }
    ))
}

#[action]
async fn scope_param_regex(req: Request, id: String) -> Result<impl Response, Error> {
    Ok(format!("scope_param_regex {}", id))
}

#[action]
async fn scope_param_opt_regex(req: Request, id: Option<String>) -> Result<impl Response, Error> {
    Ok(format!(
        "scope_param_opt_regex {}",
        match id {
            Some(x) => x,
            None => "".to_string(),
        }
    ))
}

#[action]
async fn nested_scope(req: Request, foo: String, bar: String) -> Result<impl Response, Error> {
    Ok(format!("nested_scope {} {}", foo, bar))
}

#[action]
async fn multi_params(req: Request, foo: String, bar: String) -> Result<impl Response, Error> {
    Ok(format!("multi_params {} {}", foo, bar))
}

fn router(r: &mut Router) {
    r.pipe(Pipe::new("common").and(HeadersDefault::empty().add("x-powered-by", "reign")));
    r.pipe(Pipe::new("app").and(HeadersDefault::empty().add("x-content-type-options", "nosniff")));
    r.pipe(Pipe::new("timer").and(Runtime::default()));
    r.pipe(
        Pipe::new("api")
            .and(HeadersDefault::empty().add("x-version", "1.0"))
            .and(HeadersDefault::empty().add("content-type", "application/json")),
    );

    r.scope_through("", &["common", "app"], |r| {
        r.get("str", str_);
        r.get("string", string);
        r.get("response", response);

        r.get("error", error);

        r.post("post", post);
        r.put("put", put);
        r.patch("patch", patch);
        r.delete("delete", delete);

        r.any(&[Method::POST, Method::PUT], "multi_methods", multi_methods);

        r.scope("scope_static", |r| {
            r.get("", scope_static);
        });

        r.scope_through("pipe", &["timer"], |r| {
            r.get("", pipe);
        });

        r.get(Path::new().path("param").param("id"), param);
        r.get(
            Path::new().path("param_optional").param_opt("id"),
            param_opt,
        );

        r.get(
            Path::new()
                .path("param_regex")
                .param_regex("id", "[0-9]+"),
            param_regex,
        );
        r.get(
            Path::new()
                .path("param_optional_regex")
                .param_opt_regex("id", "[0-9]+"),
            param_opt_regex,
        );

        // r.get(Path::new().path("param_glob").param_glob("id"), param_glob);
        // r.get(
        //     Path::new().path("param_optional_glob").param_opt_glob("id"),
        //     param_opt_glob,
        // );

        // r.get(Path::new().path("param_typed").param::<i32>("id"), param_typed);

        r.scope(Path::new().path("scope_param").param("id"), |r| {
            r.get("bar", scope_param);
        });

        r.scope(
            Path::new().path("scope_param_optional").param_opt("id"),
            |r| {
                r.get("bar", scope_param_opt);
            },
        );

        r.scope(
            Path::new()
                .path("scope_param_regex")
                .param_regex("id", "[0-9]+"),
            |r| {
                r.get("bar", scope_param_regex);
            },
        );

        r.scope(
            Path::new()
                .path("scope_param_optional_regex")
                .param_opt_regex("id", "[0-9]+"),
            |r| {
                r.get("bar", scope_param_opt_regex);
            },
        );

        r.scope(
            Path::new().path("nested_scope").param("id"),
            |r| {
                r.scope(Path::new().path("foo").param("bar"), |r| {
                    r.get("bar", nested_scope);
                });
            },
        );

        r.get(
            Path::new()
                .path("multi_params")
                .param("foo")
                .path("foo")
                .param("bar"),
            multi_params,
        );
    });

    //     get!(
    //         "param_glob_middle" / id: Vec<String> / "foo",
    //         param_glob_middle
    //     );
    //     get!(
    //         "param_optional_glob_middle" / id: Option<Vec<String>> / "foo",
    //         param_optional_glob_middle
    //     );

    //     scope!("scope_param_glob" / id: Vec<String>, {
    //         get!("bar", scope_param_glob);
    //     });
    //     scope!("scope_param_optional_glob" / id: Option<Vec<String>>, {
    //         get!("bar", scope_param_optional_glob);
    //     });

    //     scope!("scope_param_glob_middle" / id: Vec<String> / "foo", {
    //         get!("bar", scope_param_glob_middle);
    //     });
    //     scope!(
    //         "scope_param_optional_glob_middle" / id: Option<Vec<String>> / "foo",
    //         {
    //             get!("bar", scope_param_optional_glob_middle);
    //         }
    //     );

    //     get!(
    //         "multi_globs" / foo: Vec<String> / "foobar" / bar: Vec<String>,
    //         multi_globs
    //     );

    //     // route.scope("sibling_scope/higher", |route| {
    //     //     route.get("").to(sibling_scope_higher);
    //     // });
    //     // route.scope("sibling_scope", |route| {
    //     //     route.get("higher").to(sibling_scope_common_higher);
    //     //     route.get("lower").to(sibling_scope_common_lower);
    //     //     route.get("c").to(sibling_scope_common_c);
    //     // });
    //     // route.scope("sibling_scope/lower", |route| {
    //     //     route.get("").to(sibling_scope_lower);
    //     // });
    // });
}

async fn server() {
    serve("127.0.0.1:8080", router).await.unwrap()
}

#[tokio::main]
async fn main() {
    env_logger::init();
    server().await
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
            test(StatusCode::NOT_FOUND).await
        };

        select! {
            _ =  server() => {}
            _ = client => {}
        }
    }
}
