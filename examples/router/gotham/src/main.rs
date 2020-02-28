#![feature(proc_macro_hygiene)]

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

fn root(state: State) -> (State, &'static str) {
    (state, "root")
}

fn api(state: State) -> (State, &'static str) {
    (state, "api")
}

fn account(state: State) -> (State, &'static str) {
    (state, "account")
}

fn orgs(state: State) -> (State, &'static str) {
    (state, "orgs")
}

fn repos(state: State) -> (State, &'static str) {
    (state, "repos")
}

fn users(state: State) -> (State, &'static str) {
    (state, "users")
}

fn router() -> Router {
    pipelines!(
        common: [
            RequestLogger::new(Level::Info),
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
