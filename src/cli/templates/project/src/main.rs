use reign::{boot, prelude::views, router::serve};

views!("src", "views");

mod controllers;
mod models;

mod routes;
mod error;
mod config;

#[tokio::main]
async fn main() {
    config::CONFIG.set(boot()).unwrap();

    let addr = "127.0.0.1:8080";

    serve(addr, routes::router).await.unwrap()
}
