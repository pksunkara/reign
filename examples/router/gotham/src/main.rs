#![feature(proc_macro_hygiene)]

use futures::*;
use gotham::{
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

#[action]
fn root() {
    Ok((state, "root"))
}

#[action]
fn api() {
    Ok((state, "api"))
}

#[action]
fn account() {
    Ok((state, "account"))
}

#[action]
fn orgs() {
    Ok((state, "orgs"))
}

#[action]
fn repos() {
    Ok((state, "repos"))
}

#[action]
fn users() {
    Ok((state, "users"))
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
