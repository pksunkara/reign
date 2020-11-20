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
pub struct User {
    #[model(tag(id))]
    id: i32,
    #[model(tag(name))]
    name: String,
}

#[tokio::main]
async fn main() {
    UserName::one().load().await.unwrap().unwrap().id();
}
