#![feature(proc_macro_hygiene)]

use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use reign::prelude::*;

views!("src", "views");

async fn hello(_: HttpRequest) -> impl Responder {
    let msg = "Hello World!";

    render!(app)
}

async fn server() {
    HttpServer::new(|| App::new().route("/", web::get().to(hello)))
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

    #[actix_rt::test]
    async fn test_server() {
        spawn(server());

        delay_for(Duration::from_millis(100)).await;
        let body = reqwest::get("http://localhost:8080")
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

        assert_eq!(
            &body,
            "<div>\n  <h1>Actix</h1>\n  <p>Hello World!</p>\n</div>"
        );
    }
}
