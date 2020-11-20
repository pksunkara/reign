mod schema;

use reign::prelude::*;
use serial_test::serial;

#[derive(Debug, Model)]
pub struct User {
    #[model(tag(id))]
    id: i32,
    #[model(tag(name))]
    name: String,
}

#[tokio::test(threaded_scheduler)]
#[serial]
async fn test_id() {
    schema::setup().await;

    let user = User::one().load().await.unwrap().unwrap();

    assert_eq!(user.id(), &1);
}

#[tokio::test(threaded_scheduler)]
#[serial]
async fn test_tag_id() {
    schema::setup().await;

    let user = UserId::one().load().await.unwrap().unwrap();

    assert_eq!(user.id(), &1);
}
