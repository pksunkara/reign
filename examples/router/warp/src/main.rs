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

async fn param(id: String) -> Result<impl warp::Reply, std::convert::Infallible> {
    Ok(format!("param {}", id))
}

async fn param_optional(id: String) -> Result<impl warp::Reply, std::convert::Infallible> {
    Ok(format!("param_optional {}", id))
}

struct Name(String);

impl std::str::FromStr for Name {
    type Err = ();
    fn from_str(s: &str) -> Result<Name, ()> {
        match regex::Regex::new("^[0-9]+$").unwrap().is_match(s) {
            true => Ok(Name(s.to_string())),
            _ => Err(())
        }
    }
}

async fn param_regex(id: Name) -> Result<impl warp::Reply, std::convert::Infallible> {
    Ok(format!("param_regex {}", id.0))
}

async fn param_optional_regex(id: Name) -> Result<impl warp::Reply, std::convert::Infallible> {
    Ok(format!("param_optional_regex {}", id.0))
}

async fn param_glob(id: warp::path::Tail) -> Result<impl warp::Reply, std::convert::Infallible> {
    Ok(format!("param_glob {}", id.as_str()))
}

async fn param_optional_glob(id: warp::path::Tail) -> Result<impl warp::Reply, std::convert::Infallible> {
    Ok(format!("param_optional_glob {}", id.as_str()))
}

async fn param_glob_after(id: warp::path::Tail) -> Result<impl warp::Reply, std::convert::Infallible> {
    Ok(format!("param_glob_after {}", id.as_str()))
}

async fn param_optional_glob_after(id: warp::path::Tail) -> Result<impl warp::Reply, std::convert::Infallible> {
    Ok(format!("param_optional_glob_after {}", id.as_str()))
}

async fn scope_static() -> Result<impl warp::Reply, std::convert::Infallible> {
    Ok("scope_static")
}

async fn scope_static_b() -> Result<impl warp::Reply, std::convert::Infallible> {
    Ok("scope_static_b")
}

async fn scope_param(id: String) -> Result<impl warp::Reply, std::convert::Infallible> {
    Ok(format!("scope_param {}", id))
}

async fn scope_param_b(id: String) -> Result<impl warp::Reply, std::convert::Infallible> {
    Ok(format!("scope_param_b {}", id))
}

async fn scope_param_optional(id: String) -> Result<impl warp::Reply, std::convert::Infallible> {
    Ok(format!("scope_param_optional {}", id))
}

async fn scope_param_optional_b(id: String) -> Result<impl warp::Reply, std::convert::Infallible> {
    Ok(format!("scope_param_optional_b {}", id))
}

async fn scope_param_regex(id: Name) -> Result<impl warp::Reply, std::convert::Infallible> {
    Ok(format!("scope_param_regex {}", id.0))
}

async fn scope_param_regex_b(id: Name) -> Result<impl warp::Reply, std::convert::Infallible> {
    Ok(format!("scope_param_regex_b {}", id.0))
}

async fn scope_param_optional_regex(id: Name) -> Result<impl warp::Reply, std::convert::Infallible> {
    Ok(format!("scope_param_optional_regex {}", id.0))
}

async fn scope_param_optional_regex_b(id: Name) -> Result<impl warp::Reply, std::convert::Infallible> {
    Ok(format!("scope_param_optional_regex_b {}", id.0))
}

async fn scope_param_glob(id: warp::path::Tail) -> Result<impl warp::Reply, std::convert::Infallible> {
    Ok(format!("scope_param_glob {}", id.as_str()))
}

async fn scope_param_glob_b(id: warp::path::Tail) -> Result<impl warp::Reply, std::convert::Infallible> {
    Ok(format!("scope_param_glob_b {}", id.as_str()))
}

async fn scope_param_optional_glob(id: warp::path::Tail) -> Result<impl warp::Reply, std::convert::Infallible> {
    Ok(format!("scope_param_optional_glob {}", id.as_str()))
}

async fn scope_param_optional_glob_b(id: warp::path::Tail) -> Result<impl warp::Reply, std::convert::Infallible> {
    Ok(format!("scope_param_optional_glob_b {}", id.as_str()))
}

async fn nested_scope(id: String) -> Result<impl warp::Reply, std::convert::Infallible> {
    Ok(format!("nested_scope {}", id))
}

async fn nested_scope_b(id: String) -> Result<impl warp::Reply, std::convert::Infallible> {
    Ok(format!("nested_scope_b {}", id))
}

async fn double_slashes() -> Result<impl warp::Reply, std::convert::Infallible> {
    Ok("double_slashes")
}

async fn sibling_scope_higher() -> Result<impl warp::Reply, std::convert::Infallible> {
    Ok("sibling_scope_higher")
}

async fn sibling_scope_lower() -> Result<impl warp::Reply, std::convert::Infallible> {
    Ok("sibling_scope_lower")
}

async fn sibling_scope_common_higher() -> Result<impl warp::Reply, std::convert::Infallible> {
    Ok("sibling_scope_common_higher")
}

async fn sibling_scope_common_lower() -> Result<impl warp::Reply, std::convert::Infallible> {
    Ok("sibling_scope_common_lower")
}

async fn sibling_scope_common_c() -> Result<impl warp::Reply, std::convert::Infallible> {
    Ok("sibling_scope_common_c")
}

async fn server() {

    use warp::{get, path::{end, tail, self, path}};

    let r_post = warp::path!("post").and(warp::post()).and_then(post);

    let r_put = warp::path!("put").and(warp::put()).and_then(put);
    let r_patch = warp::path!("patch").and(warp::patch()).and_then(patch);
    let r_delete = warp::path!("delete").and(warp::delete()).and_then(delete);

    let r_methods = warp::path!("methods").and(warp::post().or(warp::put()).unify()).and_then(methods);

    let r_param = warp::path!("param" / String).and(get()).and_then(param);

    let r_param_optional = warp::path!("param_optional").and(get()).and_then(|| param_optional(Default::default()));
    let r_param_optional2 = warp::path!("param_optional" / String).and(get()).and_then(param_optional);

    let r_param_regex = warp::path!("param_regex" / Name).and(get()).and_then(param_regex);

    let r_param_optional_regex = warp::path!("param_optional_regex").and(get()).and_then(|| param_optional_regex(Name(Default::default())));
    let r_param_optional_regex2 = warp::path!("param_optional_regex" / Name).and(get()).and_then(param_optional_regex);

    let r_param_glob = path("param_glob").and(tail()).and(end()).and(get()).and_then(param_glob);
    let r_param_optional_glob = path("param_optional_glob").and(tail()).and(end()).and(get()).and_then(param_optional_glob);

    let r_param_glob_after = path("param_glob_after").and(tail()).and(path("b")).and(end()).and(get()).and_then(param_glob_after);
    let r_param_optional_glob_after = path("param_optional_glob_after").and(tail()).and(path("b")).and(end()).and(get()).and_then(param_optional_glob_after);

    // .or(warp::path!("double" / / "slashes").and(get()).and_then(double_slashes))

    let s_scope_static = path("scope_static");

    let r_scope_static_b = s_scope_static.and(warp::path!("b")).and(get()).and_then(scope_static_b);
    let r_scope_static = s_scope_static.and(end()).and(get()).and_then(scope_static);

    let s_scope_param = path("scope_param").and(path::param());

    let r_scope_param_b = s_scope_param.and(warp::path!("b")).and(get()).and_then(scope_param_b);
    let r_scope_param = s_scope_param.and(end()).and(get()).and_then(scope_param);

    let s_scope_param_optional = path("scope_param_optional");

    let r_scope_param_optional_b = s_scope_param_optional.and(warp::path!("b")).and(get()).and_then(|| scope_param_optional_b(Default::default()));
    let r_scope_param_optional = s_scope_param_optional.and(end()).and(get()).and_then(|| scope_param_optional(Default::default()));

    let r_scope_param_optional_b2 = s_scope_param_optional.and(path::param()).and(warp::path!("b")).and(get()).and_then(scope_param_optional_b);
    let r_scope_param_optional2 = s_scope_param_optional.and(path::param()).and(end()).and(get()).and_then(scope_param_optional);

    let s_scope_param_regex = path("scope_param_regex").and(path::param::<Name>());

    let r_scope_param_regex_b = s_scope_param_regex.and(warp::path!("b")).and(get()).and_then(scope_param_regex_b);
    let r_scope_param_regex = s_scope_param_regex.and(end()).and(get()).and_then(scope_param_regex);

    let s_scope_param_optional_regex = path("scope_param_optional_regex");

    let r_scope_param_optional_regex_b = s_scope_param_optional_regex.and(warp::path!("b")).and(get()).and_then(|| scope_param_optional_regex_b(Name(Default::default())));
    let r_scope_param_optional_regex = s_scope_param_optional_regex.and(end()).and(get()).and_then(|| scope_param_optional_regex(Name(Default::default())));

    let r_scope_param_optional_regex_b2 = s_scope_param_optional_regex.and(warp::path!(Name / "b")).and(get()).and_then(scope_param_optional_regex_b);
    let r_scope_param_optional_regex2 = s_scope_param_optional_regex.and(warp::path!(Name)).and(get()).and_then(scope_param_optional_regex);

    let s_nested_scope = path("nested_scope").and(path::param());
    let s_nested_scope_inner = s_nested_scope.and(path("nested_scope_inner"));
    let r_nested_scope_b = s_nested_scope_inner.and(warp::path!("b")).and(get()).and_then(nested_scope_b);
    let r_nested_scope = s_nested_scope_inner.and(end()).and(get()).and_then(nested_scope);

    let s_sibling_scope_higher = path("sibling_scope").and(path("higher"));
    let r_sibling_scope_higher = s_sibling_scope_higher.and(end()).and(get()).and_then(sibling_scope_higher);
    let s_sibling_scope_common = path("sibling_scope");
    let r_sibling_scope_common_higher = s_sibling_scope_common.and(path("higher")).and(end()).and(get()).and_then(sibling_scope_common_higher);
    let r_sibling_scope_common_lower = s_sibling_scope_common.and(path("lower")).and(end()).and(get()).and_then(sibling_scope_common_lower);
    let r_sibling_scope_common_c = s_sibling_scope_common.and(path("c")).and(end()).and(get()).and_then(sibling_scope_common_c);
    let s_sibling_scope_lower = path("sibling_scope").and(path("lower"));
    let r_sibling_scope_lower = s_sibling_scope_lower.and(end()).and(get()).and_then(sibling_scope_lower);

    let app = r_post.or(r_put).or(r_patch).or(r_delete).or(r_methods)
        .or(r_param)
        .or(r_param_optional).or(r_param_optional2)
        .or(r_param_regex)
        .or(r_param_optional_regex).or(r_param_optional_regex2)
        .or(r_param_glob).or(r_param_optional_glob)
        .or(r_param_glob_after).or(r_param_optional_glob_after)
        .or(r_scope_static_b).or(r_scope_static)
        .or(r_scope_param_b).or(r_scope_param)
        .or(r_scope_param_optional_b).or(r_scope_param_optional_b2).or(r_scope_param_optional).or(r_scope_param_optional2)
        .or(r_scope_param_regex_b).or(r_scope_param_regex)
        .or(r_scope_param_optional_regex_b).or(r_scope_param_optional_regex_b2).or(r_scope_param_optional_regex).or(r_scope_param_optional_regex2)
        .or(r_nested_scope_b).or(r_nested_scope)
        .or(r_sibling_scope_higher).or(r_sibling_scope_common_higher).or(r_sibling_scope_common_lower)
        .or(r_sibling_scope_common_c).or(r_sibling_scope_lower)
        ;

    warp::serve(app).run(([127, 0, 0, 1], 8400)).await;
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
