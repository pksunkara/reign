use reign::{boot, prelude::views, router::serve};

views!("src", "views");

mod controllers;
mod models;

mod routes;

mod config;
mod error;

#[tokio::main]
async fn main() {
    boot().load::<config::App>();

    let addr = "127.0.0.1:8000";

    serve(addr, routes::router).await.unwrap()
}
