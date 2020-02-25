#![feature(proc_macro_hygiene)]

use reign::prelude::*;
use warp::Filter;

views!("src", "views");

async fn server() {
    let app = warp::path::end().map(|| {
        let msg = "Hello World!";

        render!(app)
    });

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
    use tokio::{select, time::delay_for};

    #[tokio::test]
    async fn test_server() {
        let client = async {
            delay_for(Duration::from_millis(100)).await;
            let response = reqwest::get("http://localhost:8080").await.unwrap();

            assert_eq!(
                response.text().await.unwrap(),
                "<div>\n  <h1>Warp</h1>\n  <p>Hello World!</p>\n</div>"
            );
        };

        select! {
            _ =  server() => {}
            _ = client => {}
        }
    }
}
