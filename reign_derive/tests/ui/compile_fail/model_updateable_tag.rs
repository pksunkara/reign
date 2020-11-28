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
    #[model(tag(name))]
    name: String,
}

#[tokio::main]
async fn main() {
    let user = UserName::one().load().await.unwrap();

    assert!(user.is_some());

    let user = user.unwrap();

    user.set().name("Mike".into()).save().await.unwrap();
}
