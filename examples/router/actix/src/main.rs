#![feature(proc_macro_hygiene)]

use actix_web::{middleware::Logger, App, HttpRequest, HttpServer, Responder};
use reign::{
    prelude::*,
    router::middleware::{ContentType, HeadersDefault, Runtime},
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

async fn repos(_: HttpRequest) -> impl Responder {
    "repos"
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
            HeadersDefault::empty().add("x-powered-by", "reign"),
        ],
        timer: [
            Runtime::default(),
        ],
        api: [
            HeadersDefault::empty().add("x-version", "1.0"),
        ],
    );

    HttpServer::new(|| {
        let mut app = App::new();

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
    use std::time::Duration;
    use test_examples::router::test;

    #[actix_rt::test]
    async fn test_server() {
        spawn(server());

        let client = async {
            delay_for(Duration::from_millis(100)).await;
            test().await
        };

        client.await;
    }
}
