use reign::{
    model::Database,
    prelude::{views, Config},
    Reign,
};
use reign_plugin_static::StaticPlugin;

views!("src", "views");

mod controllers;

mod models;
mod schema;

mod routes;

mod config;
mod error;

use config::App;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8000";

    Reign::build()
        .env::<App>()
        .add_plugin(Database::new(&App::get().database_url))
        .add_plugin(StaticPlugin::new("assets").dir(&["src", "assets"]))
        .serve(addr, routes::router)
        .await
}
