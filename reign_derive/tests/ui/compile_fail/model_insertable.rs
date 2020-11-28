use reign::{model::diesel, prelude::*};

mod schema {
    use reign::model::diesel;

    diesel::table! {
        users (id) {
            id -> Int4,
            name -> Varchar,
        }
    }
}

#[derive(Debug, Model)]
pub struct User {
    #[model(no_write)]
    id: i32,
    name: String,
}

#[tokio::main]
async fn main() {
    User::new().id(4).save().await.unwrap();
}
