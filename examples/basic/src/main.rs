#![feature(proc_macro_hygiene)]

#[macro_use]
extern crate diesel;

pub mod controllers;
pub mod models;

mod routes;
mod schema;

pub mod layouts {
    reign::prelude::layouts!();
}

reign::prelude::templates!("src", "views");

pub type Repo = gotham_middleware_diesel::Repo<diesel::sqlite::SqliteConnection>;

fn main() {
    reign::boot();

    let database_url = "file:sqlite.db";
    let addr = "127.0.0.1:8080";

    gotham::start(addr, routes::router(Repo::new(&database_url)));
}
