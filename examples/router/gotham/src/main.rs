#![feature(proc_macro_hygiene)]

use gotham::hyper::Response;
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
    Ok(Response::new("response".into()))
}

#[action]
fn error() {
    let value = from_str::<Value>("{name}")?;
    Ok(Response::new(to_string(&value)?.into()))
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
fn multi_methods() {
    Ok("multi_methods")
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

#[action]
fn param(id: String) {
    Ok(format!("param {}", id))
}

#[action]
fn param_optional(id: Option<String>) {
    Ok(format!(
        "param_optional {}",
        match id {
            Some(x) => x,
            None => "".to_string(),
        }
    ))
}

#[action]
fn param_regex(id: String) {
    Ok(format!("param_regex {}", id))
}

#[action]
fn param_optional_regex(id: Option<String>) {
    Ok(format!(
        "param_optional_regex {}",
        match id {
            Some(x) => x,
            None => "".to_string(),
        }
    ))
}

// #[derive(Deserialize, StateData, StaticResponseExtender)]
// struct OptPathExtractor {
//     #[serde(rename = "*")]
//     path: Option<Vec<String>>,
// }

#[action]
fn param_glob(id: Vec<String>) {
    Ok(format!("param_glob {}", id.join("/")))
}

#[action]
fn param_optional_glob(id: Option<Vec<String>>) {
    Ok(format!(
        "param_optional_glob {}",
        match id {
            Some(x) => x.join("/"),
            None => "".to_string(),
        }
    ))
}

#[action]
fn param_glob_middle(id: Vec<String>) {
    Ok(format!("param_glob_middle {}", id.join("/")))
}

#[action]
fn param_optional_glob_middle(id: Option<Vec<String>>) {
    Ok(format!(
        "param_optional_glob_middle {}",
        match id {
            Some(x) => x.join("/"),
            None => "".to_string(),
        }
    ))
}

#[action]
fn scope_param(id: String) {
    Ok(format!("scope_param {}", id))
}

#[action]
fn scope_param_optional(id: Option<String>) {
    Ok(format!(
        "scope_param_optional {}",
        match id {
            Some(x) => x,
            None => "".to_string(),
        }
    ))
}

#[action]
fn scope_param_regex(id: String) {
    Ok(format!("scope_param_regex {}", id))
}

#[action]
fn scope_param_optional_regex(id: Option<String>) {
    Ok(format!(
        "scope_param_optional_regex {}",
        match id {
            Some(x) => x,
            None => "".to_string(),
        }
    ))
}

#[action]
fn scope_param_glob(id: Vec<String>) {
    Ok(format!("scope_param_glob {}", id.join("/")))
}

#[action]
fn scope_param_optional_glob(id: Option<Vec<String>>) {
    Ok(format!(
        "scope_param_optional_glob {}",
        match id {
            Some(x) => x.join("/"),
            None => "".to_string(),
        }
    ))
}

#[action]
fn scope_param_glob_middle(id: Vec<String>) {
    Ok(format!("scope_param_glob_middle {}", id.join("/")))
}

#[action]
fn scope_param_optional_glob_middle(id: Option<Vec<String>>) {
    Ok(format!(
        "scope_param_optional_glob_middle {}",
        match id {
            Some(x) => x.join("/"),
            None => "".to_string(),
        }
    ))
}

#[action]
fn nested_scope(foo: String, bar: String) {
    Ok(format!("nested_scope {} {}", foo, bar))
}

#[action]
fn multi_params(foo: String, bar: String) {
    Ok(format!("multi_params {} {}", foo, bar))
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

    scope!("", [common, app], {
        to!(get, "str", str_);
        to!(get, "string", string);
        to!(get, "response", response);

        to!(get, "error", error);

        to!(post, "post", post);
        to!(put, "put", put);
        to!(patch, "patch", patch);
        to!(delete, "delete", delete);

        to!([post, put], "multi_methods", multi_methods);

        scope!("scope_static", {
            to!(get, "", scope_static);
        });

        scope!("pipe", [timer], {
            to!(get, "", pipe);
        });

        scope!("pipe_empty", [], {
            to!(get, "", pipe_empty);
        });

        to!(get, "param" / id, param);
        to!(get, "param_optional" / id: Option<String>, param_optional);

        to!(get, "param_regex" / id @ "[0-9]+", param_regex);
        to!(get, "param_optional_regex" / id: Option<String> @ "[0-9]+", param_optional_regex);

        to!(get, "param_glob" / id: Vec<String>, param_glob);
        to!(get,
            "param_optional_glob" / id: Option<Vec<String>>,
            param_optional_glob
        );

        to!(get,
            "param_glob_middle" / id: Vec<String> / "foo",
            param_glob_middle
        );
        to!(get,
            "param_optional_glob_middle" / id: Option<Vec<String>> / "foo",
            param_optional_glob_middle
        );

        scope!("scope_param" / id, {
            to!(get, "bar", scope_param);
        });
        scope!("scope_param_optional" / id: Option<String>, {
            to!(get, "bar", scope_param_optional);
        });

        scope!("scope_param_regex" / id @ "[0-9]+", {
            to!(get, "bar", scope_param_regex);
        });
        scope!("scope_param_optional_regex" / id: Option<String> @ "[0-9]+", {
            to!(get, "bar", scope_param_optional_regex);
        });

        scope!("scope_param_glob" / id: Vec<String>, {
            to!(get, "bar", scope_param_glob);
        });
        scope!("scope_param_optional_glob" / id: Option<Vec<String>>, {
            to!(get, "bar", scope_param_optional_glob);
        });

        scope!("scope_param_glob_middle" / id: Vec<String> / "foo", {
            to!(get, "bar", scope_param_glob_middle);
        });
        scope!(
            "scope_param_optional_glob_middle" / id: Option<Vec<String>> / "foo",
            {
                to!(get, "bar", scope_param_optional_glob_middle);
            }
        );

        scope!("nested_scope" / foo, {
            scope!("foo" / bar, {
                to!(get, "bar", nested_scope);
            });
        });

        to!(get, "multi_params" / foo / "foo" / bar, multi_params);

        // route.scope("sibling_scope/higher", |route| {
        //     route.get("").to(sibling_scope_higher);
        // });
        // route.scope("sibling_scope", |route| {
        //     route.get("higher").to(sibling_scope_common_higher);
        //     route.get("lower").to(sibling_scope_common_lower);
        //     route.get("c").to(sibling_scope_common_c);
        // });
        // route.scope("sibling_scope/lower", |route| {
        //     route.get("").to(sibling_scope_lower);
        // });

        // route.scope("multiple_param_glob/*/foobar/*")
    });
}

async fn server() {
    router("127.0.0.1:8080").await.unwrap()
}

#[tokio::main]
async fn main() {
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
            test(StatusCode::METHOD_NOT_ALLOWED).await
        };

        select! {
            _ =  server() => {}
            _ = client => {}
        }
    }
}
