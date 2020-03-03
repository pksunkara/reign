#![feature(proc_macro_hygiene)]

use reign::{
    prelude::*,
    router::middleware::{ContentType, HeadersDefault, Runtime},
};
use tide::{middleware::RequestLogger, Request, Server};

async fn root(_: Request<()>) -> &'static str {
    "root"
}

async fn api(_: Request<()>) -> &'static str {
    "api"
}

async fn account(_: Request<()>) -> &'static str {
    "account"
}

async fn orgs(_: Request<()>) -> &'static str {
    "orgs"
}

async fn repos(_: Request<()>) -> &'static str {
    "repos"
}

async fn users(_: Request<()>) -> &'static str {
    "users"
}

fn router() -> Server<()> {
    pipelines!(
        common: [
            RequestLogger::new(),
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

    let mut app = tide::new();

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

    app
}

async fn server() {
    router().listen("127.0.0.1:8080").await.unwrap();
}

#[tokio::main]
async fn main() {
    server().await;
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
