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
fn methods() {
    Ok("methods")
}

use gotham::state::FromState;
use gotham_derive::*;
use serde::Deserialize;

#[derive(Deserialize, StateData, StaticResponseExtender)]
struct IdExtractor {
    id: String,
}

#[derive(Deserialize, StateData, StaticResponseExtender)]
struct PathExtractor {
    #[serde(rename = "*")]
    path: Vec<String>,
}

#[action]
fn param() {
    Ok(format!("param {}", IdExtractor::borrow_from(&state).id))
}

#[derive(Deserialize, StateData, StaticResponseExtender)]
struct OptIdExtractor {
    id: Option<String>,
}

#[derive(Deserialize, StateData, StaticResponseExtender)]
struct OptPathExtractor {
    #[serde(rename = "*")]
    path: Option<Vec<String>>,
}

#[action]
fn param_optional() {
    let id = OptIdExtractor::borrow_from(&state).id.as_ref();

    Ok(format!(
        "param_optional {}",
        match id {
            Some(x) => x,
            None => "",
        }
    ))
}

#[action]
fn param_regex() {
    Ok(format!(
        "param_regex {}",
        IdExtractor::borrow_from(&state).id
    ))
}

#[action]
fn param_optional_regex() {
    let id = OptIdExtractor::borrow_from(&state).id.as_ref();

    Ok(format!(
        "param_optional_regex {}",
        match id {
            Some(x) => x,
            None => "",
        }
    ))
}

#[action]
fn param_glob() {
    Ok(format!(
        "param_glob {}",
        PathExtractor::borrow_from(&state).path.join("/")
    ))
}

#[action]
fn param_optional_glob() {
    let id = OptPathExtractor::borrow_from(&state).path.as_ref();

    Ok(format!(
        "param_optional_glob {}",
        match id {
            Some(x) => x.join("/"),
            None => "".to_string(),
        }
    ))
}

#[action]
fn param_glob_after() {
    Ok(format!(
        "param_glob_after {}",
        PathExtractor::borrow_from(&state).path.join("/")
    ))
}

#[action]
fn param_optional_glob_after() {
    let id = OptPathExtractor::borrow_from(&state).path.as_ref();

    Ok(format!(
        "param_optional_glob_after {}",
        match id {
            Some(x) => x.join("/"),
            None => "".to_string(),
        }
    ))
}

#[action]
fn scope_static() {
    Ok("scope_static")
}

#[action]
fn scope_static_b() {
    Ok("scope_static_b")
}

#[action]
fn scope_param() {
    Ok(format!(
        "scope_param {}",
        IdExtractor::borrow_from(&state).id
    ))
}

#[action]
fn scope_param_b() {
    Ok(format!(
        "scope_param_b {}",
        IdExtractor::borrow_from(&state).id
    ))
}

#[action]
fn scope_param_optional() {
    let id = OptIdExtractor::borrow_from(&state).id.as_ref();

    Ok(format!(
        "scope_param_optional {}",
        match id {
            Some(x) => x,
            None => "",
        }
    ))
}

#[action]
fn scope_param_optional_b() {
    let id = OptIdExtractor::borrow_from(&state).id.as_ref();

    Ok(format!(
        "scope_param_optional_b {}",
        match id {
            Some(x) => x,
            None => "",
        }
    ))
}

#[action]
fn scope_param_regex() {
    Ok(format!(
        "scope_param_regex {}",
        IdExtractor::borrow_from(&state).id
    ))
}

#[action]
fn scope_param_regex_b() {
    Ok(format!(
        "scope_param_regex_b {}",
        IdExtractor::borrow_from(&state).id
    ))
}

#[action]
fn scope_param_optional_regex() {
    let id = OptIdExtractor::borrow_from(&state).id.as_ref();

    Ok(format!(
        "scope_param_optional_regex {}",
        match id {
            Some(x) => x,
            None => "",
        }
    ))
}

#[action]
fn scope_param_optional_regex_b() {
    let id = OptIdExtractor::borrow_from(&state).id.as_ref();

    Ok(format!(
        "scope_param_optional_regex_b {}",
        match id {
            Some(x) => x,
            None => "",
        }
    ))
}

#[action]
fn scope_param_glob() {
    Ok(format!(
        "scope_param_glob {}",
        PathExtractor::borrow_from(&state).path.join("/"),
    ))
}

#[action]
fn scope_param_glob_b() {
    Ok(format!(
        "scope_param_glob_b {}",
        PathExtractor::borrow_from(&state).path.join("/"),
    ))
}

#[action]
fn scope_param_optional_glob() {
    let id = OptPathExtractor::borrow_from(&state).path.as_ref();

    Ok(format!(
        "scope_param_optional_glob {}",
        match id {
            Some(x) => x.join("/"),
            None => "".to_string(),
        }
    ))
}

#[action]
fn scope_param_optional_glob_b() {
    let id = OptPathExtractor::borrow_from(&state).path.as_ref();

    Ok(format!(
        "scope_param_optional_glob_b {}",
        match id {
            Some(x) => x.join("/"),
            None => "".to_string(),
        }
    ))
}

#[action]
fn scope_param_glob_after() {
    Ok(format!(
        "scope_param_glob_after {}",
        PathExtractor::borrow_from(&state).path.join("/"),
    ))
}

#[action]
fn scope_param_glob_after_b() {
    Ok(format!(
        "scope_param_glob_after_b {}",
        PathExtractor::borrow_from(&state).path.join("/"),
    ))
}

#[action]
fn scope_param_optional_glob_after() {
    let id = OptPathExtractor::borrow_from(&state).path.as_ref();

    Ok(format!(
        "scope_param_optional_glob_after {}",
        match id {
            Some(x) => x.join("/"),
            None => "".to_string(),
        }
    ))
}

#[action]
fn scope_param_optional_glob_after_b() {
    let id = OptPathExtractor::borrow_from(&state).path.as_ref();

    Ok(format!(
        "scope_param_optional_glob_after_b {}",
        match id {
            Some(x) => x.join("/"),
            None => "".to_string(),
        }
    ))
}

#[action]
fn nested_scope() {
    Ok(format!(
        "nested_scope {}",
        IdExtractor::borrow_from(&state).id
    ))
}

#[action]
fn nested_scope_b() {
    Ok(format!(
        "nested_scope_b {}",
        IdExtractor::borrow_from(&state).id
    ))
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

    scope!("", [common, app], {
        get!("str", str_);
        get!("string", string);
        get!("response", response);

        get!("error", error);

        post!("post", post);
        put!("put", put);
        patch!("patch", patch);
        delete!("delete", delete);

        methods!([post, put], "methods", methods);

        route.get("param/:id").with_path_extractor::<IdExtractor>().to(param);

        route.get("param_optional").with_path_extractor::<OptIdExtractor>().to(param_optional);
        route.get("param_optional/:id").with_path_extractor::<OptIdExtractor>().to(param_optional);

        route.get("param_regex/:id:[0-9]+").with_path_extractor::<IdExtractor>().to(param_regex);

        route.get("param_optional_regex").with_path_extractor::<OptIdExtractor>().to(param_optional_regex);
        route.get("param_optional_regex/:id:[0-9]+").with_path_extractor::<OptIdExtractor>().to(param_optional_regex);

        route.get("param_glob/*").with_path_extractor::<PathExtractor>().to(param_glob);

        route.get("param_optional_glob").with_path_extractor::<OptPathExtractor>().to(param_optional_glob);
        route.get("param_optional_glob/*").with_path_extractor::<OptPathExtractor>().to(param_optional_glob);

        route.get("param_glob_after/*/b").with_path_extractor::<PathExtractor>().to(param_glob_after);

        route.get("param_optional_glob_after/b").with_path_extractor::<OptPathExtractor>().to(param_optional_glob_after);
        route.get("param_optional_glob_after/*/b").with_path_extractor::<OptPathExtractor>().to(param_optional_glob_after);

        route.scope("scope_static", |route| {
            route.get("b").to(scope_static_b);
            route.get("").to(scope_static);
        });

        route.scope("scope_param/:id", |route| {
            route.get("b").with_path_extractor::<IdExtractor>().to(scope_param_b);
            route.get("").with_path_extractor::<IdExtractor>().to(scope_param);
        });

        route.scope("scope_param_optional", |route| {
            route.get("b").with_path_extractor::<OptIdExtractor>().to(scope_param_optional_b);
            route.get("").with_path_extractor::<OptIdExtractor>().to(scope_param_optional);
        });

        route.scope("scope_param_optional/:id", |route| {
            route.get("b").with_path_extractor::<OptIdExtractor>().to(scope_param_optional_b);
            route.get("").with_path_extractor::<OptIdExtractor>().to(scope_param_optional);
        });

        route.scope("scope_param_regex/:id:[0-9]+", |route| {
            route.get("b").with_path_extractor::<IdExtractor>().to(scope_param_regex_b);
            route.get("").with_path_extractor::<IdExtractor>().to(scope_param_regex);
        });

        route.scope("scope_param_optional_regex", |route| {
            route.get("b").with_path_extractor::<OptIdExtractor>().to(scope_param_optional_regex_b);
            route.get("").with_path_extractor::<OptIdExtractor>().to(scope_param_optional_regex);
        });

        route.scope("scope_param_optional_regex/:id:[0-9]+", |route| {
            route.get("b").with_path_extractor::<OptIdExtractor>().to(scope_param_optional_regex_b);
            route.get("").with_path_extractor::<OptIdExtractor>().to(scope_param_optional_regex);
        });

        route.scope("scope_param_glob/*", |route| {
            route.get("b").with_path_extractor::<PathExtractor>().to(scope_param_glob_b);
            route.get("").with_path_extractor::<PathExtractor>().to(scope_param_glob);
        });

        route.scope("scope_param_optional_glob", |route| {
            route.get("b").with_path_extractor::<OptPathExtractor>().to(scope_param_optional_glob_b);
            route.get("").with_path_extractor::<OptPathExtractor>().to(scope_param_optional_glob);
        });

        route.scope("scope_param_optional_glob/*", |route| {
            route.get("b").with_path_extractor::<OptPathExtractor>().to(scope_param_optional_glob_b);
            route.get("").with_path_extractor::<OptPathExtractor>().to(scope_param_optional_glob);
        });

        route.scope("scope_param_glob_after/*/foo", |route| {
            route.get("b").with_path_extractor::<PathExtractor>().to(scope_param_glob_after_b);
            route.get("").with_path_extractor::<PathExtractor>().to(scope_param_glob_after);
        });

        route.scope("scope_param_optional_glob_after/foo", |route| {
            route.get("b").with_path_extractor::<OptPathExtractor>().to(scope_param_optional_glob_after_b);
            route.get("").with_path_extractor::<OptPathExtractor>().to(scope_param_optional_glob_after);
        });

        route.scope("scope_param_optional_glob_after/*/foo", |route| {
            route.get("b").with_path_extractor::<OptPathExtractor>().to(scope_param_optional_glob_after_b);
            route.get("").with_path_extractor::<OptPathExtractor>().to(scope_param_optional_glob_after);
        });

        route.scope("nested_scope/:id", |route| {
            route.scope("nested_scope_inner", |route| {
                route.get("b").with_path_extractor::<IdExtractor>().to(nested_scope_b);
                route.get("").with_path_extractor::<IdExtractor>().to(nested_scope);
            });
        });

        route.scope("sibling_scope/higher", |route| {
            route.get("").to(sibling_scope_higher);
        });
        route.scope("sibling_scope", |route| {
            route.get("higher").to(sibling_scope_common_higher);
            route.get("lower").to(sibling_scope_common_lower);
            route.get("c").to(sibling_scope_common_c);
        });
        route.scope("sibling_scope/lower", |route| {
            route.get("").to(sibling_scope_lower);
        });

        scope!("scope-static", {
            get!("/", scope_static);
        });

        scope!("pipe", [timer], {
            get!("/", pipe);
        });

        scope!("pipe-empty", [], {
            get!("/", pipe_empty);
        });
    });
}

async fn server() {
    router("127.0.0.1:8200").await.unwrap()
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
