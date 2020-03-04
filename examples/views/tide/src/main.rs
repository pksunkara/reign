#![feature(proc_macro_hygiene)]

use reign::prelude::*;
use serde::Serialize;

#[derive(Serialize)]
struct User {
    name: String,
}

views!("src", "views");

async fn server() {
    let mut app = tide::new();

    app.at("/").get(|_| async move {
        let msg = "Hello Tide!";

        render!(app)
    });

    app.at("/world").get(|_| async move { redirect!("/") });

    app.at("/hey").get(|_| async move {
        let msg = "Hey Tide!";

        render!(app, status = 404)
    });

    app.at("/json").get(|_| async move {
        let user = User {
            name: "Tide".to_string(),
        };

        json!(user)
    });

    app.at("/json_err").get(|_| async move {
        let user = User {
            name: "Tide".to_string(),
        };

        json!(user, status = 422)
    });

    app.listen("127.0.0.1:8080").await.unwrap();
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
            test("Tide").await
        };

        select! {
            _ =  server() => {}
            _ = client => {}
        }
    }
}
