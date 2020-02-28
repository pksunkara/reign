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
    router::middleware::{ContentType, Runtime},
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
        ],
        timer: [
            Runtime::default(),
        ],
        api: [
            ContentType::empty().json(),
        ],
    );

    build_simple_router(|route| {
        scope!("/", [common, app], {
            post!("/", root);

            scope!("/account", {
                post!("/", account);
            });

            scope!("/orgs", [], {
                post!("/", orgs);
            });

            scope!("/users", [timer], {
                get!("/", users);
            });
        });

        scope!("/api", [common, api], {
            post!("/", api);
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

            let res = client.post(url).send().await.unwrap();

            assert_eq!(res.status(), StatusCode::OK);
            assert_eq!(res.text().await.unwrap(), "account");

            let res = client
                .post(url)
                .header(CONTENT_TYPE, "application/json")
                .send()
                .await
                .unwrap();

            assert_eq!(res.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
            assert_eq!(res.text().await.unwrap(), "");

            url = "http://localhost:8080/orgs";

            let res = client.post(url).send().await.unwrap();

            assert_eq!(res.status(), StatusCode::OK);
            assert_eq!(res.text().await.unwrap(), "orgs");

            let res = client
                .post(url)
                .header(CONTENT_TYPE, "application/json")
                .send()
                .await
                .unwrap();

            assert_eq!(res.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
            assert_eq!(res.text().await.unwrap(), "");

            url = "http://localhost:8080/users";

            let res = client.get(url).send().await.unwrap();

            assert_eq!(res.status(), StatusCode::OK);
            assert!(res.headers().contains_key("x-runtime"));
            assert_eq!(res.text().await.unwrap(), "users");

            url = "http://localhost:8080/api";

            let res = client.post(url).send().await.unwrap();

            assert_eq!(res.status(), StatusCode::OK);
            assert_eq!(res.text().await.unwrap(), "api");

            let res = client
                .post(url)
                .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
                .send()
                .await
                .unwrap();

            assert_eq!(res.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
            assert_eq!(res.text().await.unwrap(), "");
        };

        select! {
            _ =  server() => {}
            _ = client => {}
        }
    }
}
