use reign::{boot, prelude::views, router::serve};

mod controllers;
mod models;

mod routes;
mod errors;
mod config;

#[tokio::main]
async fn main() {
    config::CONFIG.set(boot()).unwrap();

    let addr = "127.0.0.1:8080";

    serve(addr, routes::router).await.unwrap()
}
