#![feature(proc_macro_hygiene)]

use gotham::{
    hyper::Response,
    init_server,
    middleware::logger::RequestLogger,
    router::{builder::*, Router},
    state::State,
};
use reign::{
    log::Level,
    prelude::*,
    router::middleware::{ContentType, HeadersDefault, Runtime},
};
use serde_json::{from_str, to_string, Value};

mod errors;

#[action]
fn root() {
    Ok("root")
}

#[action]
fn api() {
    Ok("api".to_string())
}

#[action]
fn account() {
    Ok("account")
}

#[action]
fn orgs() {
    Ok("orgs")
}

#[action]
fn repos() {
    Ok("repos")
}

#[action]
fn users() {
    Ok(Response::new("users".into()))
}

#[action]
fn error() {
    let value = from_str::<Value>("{name}")?;
    Ok(Response::new(to_string(&value)?.into()))
}

fn router() -> Router {
    pipelines!(
        common: [
            RequestLogger::new(Level::Info),
        ],
        app: [
            ContentType::empty().form(),
            HeadersDefault::empty().add("x-powered-by", "reign"),
        ],
        timer: [
            Runtime::default(),
        ],
        api: [
            HeadersDefault::empty().add("x-version", "1.0"),
        ],
    );

    build_simple_router(|route| {
        scope!("/", [common, app], {
            post!("/", root);
            get!("/error", error);

            scope!("/account", {
                get!("/", account);
            });

            scope!("/orgs", [], {
                get!("/", orgs);

                scope!("/repos", {
                    get!("/", repos);
                });
            });

            scope!("/users", [timer], {
                get!("/", users);
            });
        });

        scope!("/api", [common, api], {
            get!("/", api);
        });
    })
}

async fn server() {
    init_server("127.0.0.1:8080", router()).await.unwrap()
}

#[tokio::main]
async fn main() {
    server().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use test_examples::router::test;
    use tokio::{select, time::delay_for};

    #[tokio::test]
    async fn test_server() {
        let client = async {
            delay_for(Duration::from_millis(100)).await;
            test().await
        };

        select! {
            _ =  server() => {}
            _ = client => {}
        }
    }
}
