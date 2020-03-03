#![feature(proc_macro_hygiene)]

use gotham::{
    hyper::{Body, Response},
    init_server,
    router::{builder::*, Router},
    state::State,
};
use reign::prelude::*;

views!("src", "views");

fn hello(state: State) -> (State, Response<Body>) {
    let msg = "Hello World!";

    (state, render!(app))
}

fn world(state: State) -> (State, Response<Body>) {
    (state, redirect!("/"))
}

fn router() -> Router {
    build_simple_router(|route| {
        route.get("/").to(hello);
        route.get("/world").to(world);
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
    use reqwest::{redirect::Policy, Client, StatusCode};
    use std::time::Duration;
    use tokio::{select, time::delay_for};

    #[tokio::test]
    async fn test_server() {
        let client = async {
            delay_for(Duration::from_millis(100)).await;
            let client = Client::builder().redirect(Policy::none()).build().unwrap();

            let response = client.get("http://localhost:8080").send().await.unwrap();

            assert_eq!(
                response.text().await.unwrap(),
                "<div>\n  <h1>Gotham</h1>\n  <p>Hello World!</p>\n</div>"
            );

            let response = client
                .get("http://localhost:8080/world")
                .send()
                .await
                .unwrap();

            assert_eq!(response.status(), StatusCode::SEE_OTHER);
        };

        select! {
            _ =  server() => {}
            _ = client => {}
        }
    }
}
