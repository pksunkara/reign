mod schema;

use reign::{model::diesel, prelude::*};
use serial_test::serial;

#[derive(Debug, Model)]
pub struct User {
    #[model(tag(id))]
    id: i32,
    name: String,
}

#[derive(Debug, Model)]
#[model(primary_key(id, name), table_name = users)]
pub struct Foo {
    id: i32,
    name: String,
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn test_id() {
    schema::setup().await;

    let user = User::one().await.unwrap().unwrap();

    assert_eq!(user.id(), &1);
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn test_tag_id() {
    schema::setup().await;

    let user = UserId::one().await.unwrap().unwrap();

    assert_eq!(user.id(), &1);
}

#[tokio::test(flavor = "multi_thread")]
#[serial]
async fn test_id_with_multi_columns() {
    schema::setup().await;

    let user = Foo::one().await.unwrap().unwrap();

    assert_eq!(user.id(), (&1, &"John".to_string()));
}
