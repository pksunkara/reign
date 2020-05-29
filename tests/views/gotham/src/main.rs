#![feature(proc_macro_hygiene)]

use gotham::{
    hyper::{Body, Response},
    init_server,
    router::{builder::*, Router},
    state::State,
};
use reign::prelude::*;
use serde::Serialize;

#[derive(Serialize)]
struct User {
    name: String,
}

views!("src", "views");

fn hey(state: State) -> (State, Response<Body>) {
    let msg = "Hey Gotham!";

    (state, render!(app, status = 404))
}

fn json_err(state: State) -> (State, Response<Body>) {
    let user = User {
        name: "Gotham".to_string(),
    };

    (state, json!(user, status = 422))
}

fn router() -> Router {
    build_simple_router(|route| {
        route.get("/hey").to(hey);
        route.get("/json_err").to(json_err);
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
    use test_integrations::views::test;
    use tokio::{select, time::delay_for};

    #[tokio::test]
    async fn test_server() {
        let client = async {
            delay_for(Duration::from_millis(100)).await;
            test("Gotham").await
        };

        select! {
            _ =  server() => {}
            _ = client => {}
        }
    }
}
