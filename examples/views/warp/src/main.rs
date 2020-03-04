#![feature(proc_macro_hygiene)]

use reign::prelude::*;
use warp::Filter;

views!("src", "views");

async fn server() {
    let hello = warp::path::end().map(|| {
        let msg = "Hello World!";

        render!(app)
    });

    let world = warp::path("world").map(|| redirect!("/"));

    let hey = warp::path("hey").map(|| {
        let msg = "Hey!";

        render!(app, status = 404)
    });

    let app = hello.or(world.or(hey));

    warp::serve(app).run(([127, 0, 0, 1], 8080)).await
}

#[tokio::main]
async fn main() {
    server().await;
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
