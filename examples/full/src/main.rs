reign::prelude::views!("src", "views");

pub mod controllers;
pub mod models;

mod routes;
mod schema;

pub type Repo = gotham_middleware_diesel::Repo<diesel::sqlite::SqliteConnection>;

#[tokio::main]
async fn main() {
    reign::boot();

    let database_url = "file:sqlite.db";
    let addr = "127.0.0.1:8080";

    routes::router(addr).await.unwrap();
}
