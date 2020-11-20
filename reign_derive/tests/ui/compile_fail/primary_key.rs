use reign::{model::diesel, prelude::*};

mod schema {
    diesel::table! {
        users (id) {
            id -> Int4,
            name -> Varchar,
        }
    }
}

#[derive(Debug, Model)]
#[model(primary_key(id, email))]
pub struct User {
    id: i32,
    name: String,
}

fn main() {}
