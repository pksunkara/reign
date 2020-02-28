#![feature(proc_macro_hygiene)]

use actix_web::{middleware::Logger, App, HttpRequest, HttpServer, Responder};
use reign::{
    prelude::*,
    router::middleware::{ContentType, Runtime},
};

async fn root(_: HttpRequest) -> impl Responder {
    "root"
}

async fn api(_: HttpRequest) -> impl Responder {
    "api"
}

async fn account(_: HttpRequest) -> impl Responder {
    "account"
}

async fn orgs(_: HttpRequest) -> impl Responder {
    "orgs"
}

async fn users(_: HttpRequest) -> impl Responder {
    "users"
}

async fn server() {
    pipelines!(
        common: [
            // Logger::default(),
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

    HttpServer::new(|| {
        let mut app = App::new();

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

        app
    })
    .bind("127.0.0.1:8080")
    .unwrap()
    .run()
    .await
    .unwrap()
}

#[actix_rt::main]
async fn main() {
    server().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_rt::{spawn, time::delay_for};
    use reqwest::{header::CONTENT_TYPE, Client, StatusCode};
    use std::time::Duration;

    #[actix_rt::test]
    async fn test_server() {
        spawn(server());

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

        client.await;
    }
}
