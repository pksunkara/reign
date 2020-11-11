use reign::{
    model::DatabasePlugin,
    prelude::{views, Config},
    Reign,
};

views!("src", "views");

mod controllers;
mod models;

mod routes;

mod config;
mod error;

use config::App;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8000";

    Reign::build()
        .env::<App>()
        .add_plugin(DatabasePlugin::new(&App::get().database_url))
        .serve(addr, routes::router)
        .await
}
