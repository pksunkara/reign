#![feature(proc_macro_hygiene)]

use reign::prelude::*;

views!("src", "views");

async fn server() {
    let mut app = tide::new();

    app.at("/").get(|_| async move {
        let msg = "Hello World!";

        render!(app)
    });

    app.at("/world").get(|_| async move { redirect!("/") });

    app.listen("127.0.0.1:8080").await.unwrap();
}

#[tokio::main]
async fn main() {
    server().await;
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
                "<div>\n  <h1>Tide</h1>\n  <p>Hello World!</p>\n</div>"
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
