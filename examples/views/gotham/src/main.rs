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
    use std::time::Duration;
    use test_examples::views::test;
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
