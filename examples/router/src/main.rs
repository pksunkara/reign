use reign::{
    prelude::*,
    router::{
        hyper::Response as Res, middleware::HeadersDefault, serve, Path, Pipe, Request, Response,
        Router,
    },
};
use serde_json::{from_str, to_string, Value};

mod errors;

use errors::Error;

#[action]
async fn str_(_req: &mut Request) -> Result<impl Response, Error> {
    Ok("str")
}

#[action]
async fn string(_req: &mut Request) -> Result<impl Response, Error> {
    Ok("string".to_string())
}

#[action]
async fn response(_req: &mut Request) -> Result<impl Response, Error> {
    Ok(Res::new("response".into()))
}

#[action]
async fn error(_req: &mut Request) -> Result<impl Response, Error> {
    let value = from_str::<Value>("{name}")?;
    Ok(Res::new(to_string(&value)?.into()))
}

#[action]
async fn param(_req: &mut Request, id: String) -> Result<impl Response, Error> {
    Ok(format!("param {}", id))
}

#[action]
async fn param_opt(_req: &mut Request, id: Option<String>) -> Result<impl Response, Error> {
    Ok(format!(
        "param_opt {}",
        match id {
            Some(x) => x,
            None => "".to_string(),
        }
    ))
}

#[action]
async fn param_regex(_req: &mut Request, id: String) -> Result<impl Response, Error> {
    Ok(format!("param_regex {}", id))
}

#[action]
async fn param_opt_regex(_req: &mut Request, id: Option<String>) -> Result<impl Response, Error> {
    Ok(format!(
        "param_opt_regex {}",
        match id {
            Some(x) => x,
            None => "".to_string(),
        }
    ))
}

#[action]
async fn scope_param(_req: &mut Request, id: String) -> Result<impl Response, Error> {
    Ok(format!("scope_param {}", id))
}

#[action]
async fn scope_param_opt(_req: &mut Request, id: Option<String>) -> Result<impl Response, Error> {
    Ok(format!(
        "scope_param_opt {}",
        match id {
            Some(x) => x,
            None => "".to_string(),
        }
    ))
}

#[action]
async fn scope_param_regex(_req: &mut Request, id: String) -> Result<impl Response, Error> {
    Ok(format!("scope_param_regex {}", id))
}

#[action]
async fn scope_param_opt_regex(
    _req: &mut Request,
    id: Option<String>,
) -> Result<impl Response, Error> {
    Ok(format!(
        "scope_param_opt_regex {}",
        match id {
            Some(x) => x,
            None => "".to_string(),
        }
    ))
}

#[action]
async fn nested_scope(
    _req: &mut Request,
    foo: String,
    bar: String,
) -> Result<impl Response, Error> {
    Ok(format!("nested_scope {} {}", foo, bar))
}

#[action]
async fn multi_params(
    _req: &mut Request,
    foo: String,
    bar: String,
) -> Result<impl Response, Error> {
    Ok(format!("multi_params {} {}", foo, bar))
}

fn router(r: &mut Router) {
    r.pipe(Pipe::new("common").add(HeadersDefault::empty().add("x-powered-by", "reign")));
    r.pipe(Pipe::new("app").add(HeadersDefault::empty().add("x-content-type-options", "nosniff")));
    r.pipe(
        Pipe::new("api")
            .add(HeadersDefault::empty().add("x-version", "1.0"))
            .add(HeadersDefault::empty().add("content-type", "application/json")),
    );

    r.scope_through("", &["common", "app"], |r| {
        r.get("str", str_);
        r.get("string", string);
        r.get("response", response);

        r.get("error", error);

        r.get(p!("param" / id), param);
        r.get(p!("param_opt" / id?), param_opt);

        r.get(p!("param_regex" / id @ "[0-9]+"), param_regex);
        r.get(p!("param_opt_regex" / id? @ "[0-9]+"), param_opt_regex);

        // r.get(Path::new().path("param_glob").param_glob("id"), param_glob);
        // r.get(
        //     Path::new().path("param_opt_glob").param_opt_glob("id"),
        //     param_opt_glob,
        // );

        // r.get(Path::new().path("param_typed").param::<i32>("id"), param_typed);

        r.scope(p!("scope_param" / id), |r| {
            r.get("bar", scope_param);
        });

        r.scope(p!("scope_param_opt" / id?), |r| {
            r.get("bar", scope_param_opt);
        });

        r.scope(p!("scope_param_regex" / id @ "[0-9]+"), |r| {
            r.get("bar", scope_param_regex);
        });

        r.scope(p!("scope_param_opt_regex" / id? @ "[0-9]+"), |r| {
            r.get("bar", scope_param_opt_regex);
        });

        r.scope(p!("nested_scope" / foo), |r| {
            r.scope(p!("foo" / bar), |r| {
                r.get("bar", nested_scope);
            });
        });

        r.get(p!("multi_params" / foo / "foo" / bar), multi_params);
    });

    //     get!(
    //         "param_glob_middle" / id: Vec<String> / "foo",
    //         param_glob_middle
    //     );
    //     get!(
    //         "param_opt_glob_middle" / id: Option<Vec<String>> / "foo",
    //         param_opt_glob_middle
    //     );

    //     scope!("scope_param_glob" / id: Vec<String>, {
    //         get!("bar", scope_param_glob);
    //     });
    //     scope!("scope_param_opt_glob" / id: Option<Vec<String>>, {
    //         get!("bar", scope_param_opt_glob);
    //     });

    //     scope!("scope_param_glob_middle" / id: Vec<String> / "foo", {
    //         get!("bar", scope_param_glob_middle);
    //     });
    //     scope!(
    //         "scope_param_opt_glob_middle" / id: Option<Vec<String>> / "foo",
    //         {
    //             get!("bar", scope_param_opt_glob_middle);
    //         }
    //     );

    //     get!(
    //         "multi_globs" / foo: Vec<String> / "foobar" / bar: Vec<String>,
    //         multi_globs
    //     );

    // In router
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
    server().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::{Client, StatusCode};
    use std::time::Duration;
    use tokio::{select, time::delay_for};

    #[tokio::test]
    async fn test_server() {
        let client = async {
            delay_for(Duration::from_millis(100)).await;

            let mut url;
            let client = Client::new();

            url = "http://localhost:8080/str";

            let res = client.get(url).send().await.unwrap();

            assert_eq!(res.status(), StatusCode::OK);
            assert!(res.headers().contains_key("x-powered-by"));
            assert!(res.headers().contains_key("x-content-type-options"));
            assert_eq!(res.text().await.unwrap(), "str");

            url = "http://localhost:8080/string";

            let res = client.get(url).send().await.unwrap();

            assert_eq!(res.status(), StatusCode::OK);
            assert!(res.headers().contains_key("x-powered-by"));
            assert!(res.headers().contains_key("x-content-type-options"));
            assert_eq!(res.text().await.unwrap(), "string");

            url = "http://localhost:8080/response";

            let res = client.get(url).send().await.unwrap();

            assert_eq!(res.status(), StatusCode::OK);
            assert!(res.headers().contains_key("x-powered-by"));
            assert!(res.headers().contains_key("x-content-type-options"));
            assert_eq!(res.text().await.unwrap(), "response");

            url = "http://localhost:8080/error";

            let res = client.get(url).send().await.unwrap();

            assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);
            assert_eq!(res.text().await.unwrap(), "");

            url = "http://localhost:8080/param/foobar";

            let res = client.get(url).send().await.unwrap();

            assert_eq!(res.status(), StatusCode::OK);
            assert!(res.headers().contains_key("x-powered-by"));
            assert!(res.headers().contains_key("x-content-type-options"));
            assert_eq!(res.text().await.unwrap(), "param foobar");

            url = "http://localhost:8080/param_opt/foobar";

            let res = client.get(url).send().await.unwrap();

            assert_eq!(res.status(), StatusCode::OK);
            assert!(res.headers().contains_key("x-powered-by"));
            assert!(res.headers().contains_key("x-content-type-options"));
            assert_eq!(res.text().await.unwrap(), "param_opt foobar");

            url = "http://localhost:8080/param_opt";

            let res = client.get(url).send().await.unwrap();

            assert_eq!(res.status(), StatusCode::OK);
            assert!(res.headers().contains_key("x-powered-by"));
            assert!(res.headers().contains_key("x-content-type-options"));
            assert_eq!(res.text().await.unwrap(), "param_opt ");

            url = "http://localhost:8080/param_regex/123";

            let res = client.get(url).send().await.unwrap();

            assert_eq!(res.status(), StatusCode::OK);
            assert!(res.headers().contains_key("x-powered-by"));
            assert!(res.headers().contains_key("x-content-type-options"));
            assert_eq!(res.text().await.unwrap(), "param_regex 123");

            url = "http://localhost:8080/param_regex/foobar";

            let res = client.get(url).send().await.unwrap();

            assert_eq!(res.status(), StatusCode::NOT_FOUND);
            assert_eq!(res.text().await.unwrap(), "");

            url = "http://localhost:8080/param_opt_regex/123";

            let res = client.get(url).send().await.unwrap();

            assert_eq!(res.status(), StatusCode::OK);
            assert!(res.headers().contains_key("x-powered-by"));
            assert!(res.headers().contains_key("x-content-type-options"));
            assert_eq!(res.text().await.unwrap(), "param_opt_regex 123");

            url = "http://localhost:8080/param_opt_regex";

            let res = client.get(url).send().await.unwrap();

            assert_eq!(res.status(), StatusCode::OK);
            assert!(res.headers().contains_key("x-powered-by"));
            assert!(res.headers().contains_key("x-content-type-options"));
            assert_eq!(res.text().await.unwrap(), "param_opt_regex ");

            url = "http://localhost:8080/param_opt_regex/foobar";

            let res = client.get(url).send().await.unwrap();

            assert_eq!(res.status(), StatusCode::NOT_FOUND);
            assert_eq!(res.text().await.unwrap(), "");

            url = "http://localhost:8080/scope_param/foobar/bar";

            let res = client.get(url).send().await.unwrap();

            assert_eq!(res.status(), StatusCode::OK);
            assert!(res.headers().contains_key("x-powered-by"));
            assert!(res.headers().contains_key("x-content-type-options"));
            assert_eq!(res.text().await.unwrap(), "scope_param foobar");

            url = "http://localhost:8080/scope_param_opt/foobar/bar";

            let res = client.get(url).send().await.unwrap();

            assert_eq!(res.status(), StatusCode::OK);
            assert!(res.headers().contains_key("x-powered-by"));
            assert!(res.headers().contains_key("x-content-type-options"));
            assert_eq!(res.text().await.unwrap(), "scope_param_opt foobar");

            url = "http://localhost:8080/scope_param_opt/bar";

            let res = client.get(url).send().await.unwrap();

            assert_eq!(res.status(), StatusCode::OK);
            assert!(res.headers().contains_key("x-powered-by"));
            assert!(res.headers().contains_key("x-content-type-options"));
            assert_eq!(res.text().await.unwrap(), "scope_param_opt ");

            url = "http://localhost:8080/scope_param_regex/123/bar";

            let res = client.get(url).send().await.unwrap();

            assert_eq!(res.status(), StatusCode::OK);
            assert!(res.headers().contains_key("x-powered-by"));
            assert!(res.headers().contains_key("x-content-type-options"));
            assert_eq!(res.text().await.unwrap(), "scope_param_regex 123");

            url = "http://localhost:8080/scope_param_regex/foobar/bar";

            let res = client.get(url).send().await.unwrap();

            assert_eq!(res.status(), StatusCode::NOT_FOUND);
            assert_eq!(res.text().await.unwrap(), "");

            url = "http://localhost:8080/scope_param_opt_regex/123/bar";

            let res = client.get(url).send().await.unwrap();

            assert_eq!(res.status(), StatusCode::OK);
            assert!(res.headers().contains_key("x-powered-by"));
            assert!(res.headers().contains_key("x-content-type-options"));
            assert_eq!(res.text().await.unwrap(), "scope_param_opt_regex 123");

            url = "http://localhost:8080/scope_param_opt_regex/bar";

            let res = client.get(url).send().await.unwrap();

            assert_eq!(res.status(), StatusCode::OK);
            assert!(res.headers().contains_key("x-powered-by"));
            assert!(res.headers().contains_key("x-content-type-options"));
            assert_eq!(res.text().await.unwrap(), "scope_param_opt_regex ");

            url = "http://localhost:8080/scope_param_opt_regex/foobar/bar";

            let res = client.get(url).send().await.unwrap();

            assert_eq!(res.status(), StatusCode::NOT_FOUND);
            assert_eq!(res.text().await.unwrap(), "");

            url = "http://localhost:8080/nested_scope/123/foo/456/bar";

            let res = client.get(url).send().await.unwrap();

            assert_eq!(res.status(), StatusCode::OK);
            assert!(res.headers().contains_key("x-powered-by"));
            assert!(res.headers().contains_key("x-content-type-options"));
            assert_eq!(res.text().await.unwrap(), "nested_scope 123 456");

            url = "http://localhost:8080/multi_params/123/foo/456";

            let res = client.get(url).send().await.unwrap();

            assert_eq!(res.status(), StatusCode::OK);
            assert!(res.headers().contains_key("x-powered-by"));
            assert!(res.headers().contains_key("x-content-type-options"));
            assert_eq!(res.text().await.unwrap(), "multi_params 123 456");
        };

        select! {
            _ =  server() => {}
            _ = client => {}
        }
    }
}
