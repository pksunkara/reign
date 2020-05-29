#![feature(proc_macro_hygiene)]

use reign::prelude::*;
use serde::Serialize;
use warp::Filter;

#[derive(Serialize)]
struct User {
    name: String,
}

views!("src", "views");

async fn server() {
    let hey = warp::path("hey").map(|| {
        let msg = "Hey Warp!";

        render!(app, status = 404)
    });

    let json_err = warp::path("json_err").map(|| {
        let user = User {
            name: "Warp".to_string(),
        };

        json!(user, status = 422)
    });

    let app = hey.or(json_err);

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
    use test_integrations::views::test;
    use tokio::{select, time::delay_for};

    #[tokio::test]
    async fn test_server() {
        let client = async {
            delay_for(Duration::from_millis(100)).await;
            test("Warp").await
        };

        select! {
            _ =  server() => {}
            _ = client => {}
        }
    }
}
