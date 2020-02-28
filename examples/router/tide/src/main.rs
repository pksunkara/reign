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
            HeadersDefault::new().add("x-powered-by", "reign"),
        ],
        timer: [
            Runtime::default(),
        ],
        api: [
            HeadersDefault::new().add("x-version", "1.0"),
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
    use reqwest::{header::CONTENT_TYPE, Client, StatusCode};
    use std::time::Duration;
    use tokio::{select, time::delay_for};

    #[tokio::test]
    async fn test_server() {
        let client = async {
            let mut url = "http://localhost:8080";

            delay_for(Duration::from_millis(100)).await;
            let client = Client::new();

            let res = client.post(url).send().await.unwrap();

            assert_eq!(res.status(), StatusCode::OK);
            assert!(res.headers().contains_key("x-powered-by"));
            assert_eq!(res.text().await.unwrap(), "root");

            let res = client
                .post(url)
                .header(CONTENT_TYPE, "application/json")
                .send()
                .await
                .unwrap();

            assert_eq!(res.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
            assert_eq!(res.text().await.unwrap(), "");

            url = "http://localhost:8080/account";

            let res = client.get(url).send().await.unwrap();

            assert_eq!(res.status(), StatusCode::OK);
            assert!(res.headers().contains_key("x-powered-by"));
            assert_eq!(res.text().await.unwrap(), "account");

            url = "http://localhost:8080/orgs";

            let res = client.get(url).send().await.unwrap();

            assert_eq!(res.status(), StatusCode::OK);
            assert!(res.headers().contains_key("x-powered-by"));
            assert_eq!(res.text().await.unwrap(), "orgs");

            url = "http://localhost:8080/orgs/repos";

            let res = client.get(url).send().await.unwrap();

            assert_eq!(res.status(), StatusCode::OK);
            assert!(res.headers().contains_key("x-powered-by"));
            assert_eq!(res.text().await.unwrap(), "repos");

            url = "http://localhost:8080/users";

            let res = client.get(url).send().await.unwrap();

            assert_eq!(res.status(), StatusCode::OK);
            assert!(res.headers().contains_key("x-powered-by"));
            assert!(res.headers().contains_key("x-runtime"));
            assert_eq!(res.text().await.unwrap(), "users");

            url = "http://localhost:8080/api";

            let res = client.get(url).send().await.unwrap();

            assert_eq!(res.status(), StatusCode::OK);
            assert!(res.headers().contains_key("x-version"));
            assert_eq!(res.text().await.unwrap(), "api");
        };

        select! {
            _ =  server() => {}
            _ = client => {}
        }
    }
}
