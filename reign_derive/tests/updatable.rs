mod schema;

use reign::{model::diesel, prelude::*};
use serial_test::serial;

#[derive(Debug, Model)]
pub struct User {
    #[model(tag(id))]
    id: i32,
    name: String,
}

#[tokio::test(threaded_scheduler)]
#[serial]
async fn test_change() {
    schema::setup().await;

    let change = User::change();
}

#[tokio::test(threaded_scheduler)]
#[serial]
async fn test_instance_change() {
    schema::setup().await;

    let user = User::one().load().await.unwrap().unwrap();

    let change = user;
}
