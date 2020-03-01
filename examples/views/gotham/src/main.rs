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

fn router() -> Router {
    build_simple_router(|route| {
        route.get("/").to(hello);
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
    use tokio::{select, time::delay_for};

    #[tokio::test]
    async fn test_server() {
        let client = async {
            delay_for(Duration::from_millis(100)).await;
            let response = reqwest::get("http://localhost:8080").await.unwrap();

            assert_eq!(
                response.text().await.unwrap(),
                "<div>\n  <h1>Gotham</h1>\n  <p>Hello World!</p>\n</div>"
            );
        };

        select! {
            _ =  server() => {}
            _ = client => {}
        }
    }
}
