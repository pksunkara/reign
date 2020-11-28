mod schema;

use reign::{model::diesel, prelude::*};
use serial_test::serial;

#[derive(Debug, Model)]
pub struct User {
    #[model(no_write, tag(id))]
    id: i32,
    name: String,
    email: Option<String>,
}

#[tokio::test(threaded_scheduler)]
#[serial]
async fn test_new() {
    schema::setup().await;
    let one = User::new()
        .name("Ray".into())
        .email(Some("ray@mail.com".into()))
        .save()
        .await
        .unwrap();

    assert_eq!(one.id, 4);
    assert_eq!(one.name, "Ray");
    assert_eq!(one.email, Some("ray@mail.com".into()));
}

#[tokio::test(threaded_scheduler)]
#[serial]
async fn test_new_default() {
    schema::setup().await;
    let one = User::new()
        .email(Some("ray@mail.com".into()))
        .save()
        .await
        .unwrap();

    assert_eq!(one.id, 4);
    assert_eq!(one.name, "Mike");
    assert_eq!(one.email, Some("ray@mail.com".into()));
}

#[tokio::test(threaded_scheduler)]
#[serial]
async fn test_new_default_nullable() {
    schema::setup().await;
    let one = User::new().name("Ray".into()).save().await.unwrap();

    assert_eq!(one.id, 4);
    assert_eq!(one.name, "Ray");
    assert_eq!(one.email, Some("mike@mail.com".into()));
}

#[tokio::test(threaded_scheduler)]
#[serial]
async fn test_new_null_in_default_nullable() {
    schema::setup().await;
    let one = User::new()
        .name("Ray".into())
        .email(None)
        .save()
        .await
        .unwrap();

    assert_eq!(one.id, 4);
    assert_eq!(one.name, "Ray");
    assert_eq!(one.email, None);
}

#[tokio::test(threaded_scheduler)]
#[serial]
async fn test_tag_new() {
    schema::setup().await;
    let one = UserId::new()
        .name("Ray".into())
        .email(Some("ray@mail.com".into()))
        .save()
        .await
        .unwrap();

    assert_eq!(one.id, 4);
}
