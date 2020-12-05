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
async fn test_delete() {
    schema::setup().await;

    let changes = User::delete().name("John").drop().await.unwrap();

    assert_eq!(changes.len(), 2);
    assert_eq!(changes[0].id, 1);
    assert_eq!(changes[0].name, "John");
    assert_eq!(changes[0].email, None);
    assert_eq!(changes[1].id, 3);
    assert_eq!(changes[1].name, "John");
    assert_eq!(changes[1].email, Some("john@mail.com".into()));

    // Check that it is saved in DB
    let all = User::all().load().await.unwrap();

    assert_eq!(all.len(), 1);
    assert_eq!(all[0].id, 2);
    assert_eq!(all[0].name, "Sean");
    assert_eq!(all[0].email, Some("sean@mail.com".into()));
}

#[tokio::test(threaded_scheduler)]
#[serial]
async fn test_drop() {
    schema::setup().await;

    let user = User::one().id(3).load().await.unwrap();

    assert!(user.is_some());

    let user = user.unwrap();

    assert_eq!(user.id, 3);
    assert_eq!(user.name, "John");
    assert_eq!(user.email, Some("john@mail.com".into()));

    let user = user.drop().await.unwrap();

    assert_eq!(user.id, 3);
    assert_eq!(user.name, "John");
    assert_eq!(user.email, Some("john@mail.com".into()));

    // Check that it is saved in DB
    let user = User::one().id(3).load().await.unwrap();

    assert!(user.is_none());
}

#[tokio::test(threaded_scheduler)]
#[serial]
async fn test_tag_delete() {
    schema::setup().await;

    let changes = UserId::delete().name("John").drop().await.unwrap();

    assert_eq!(changes.len(), 2);
    assert_eq!(changes[0].id, 1);
    assert_eq!(changes[1].id, 3);

    // Check that it is saved in DB
    let all = User::all().load().await.unwrap();

    assert_eq!(all.len(), 1);
    assert_eq!(all[0].id, 2);
    assert_eq!(all[0].name, "Sean");
    assert_eq!(all[0].email, Some("sean@mail.com".into()));
}

#[tokio::test(threaded_scheduler)]
#[serial]
async fn test_tag_set() {
    schema::setup().await;

    let user = UserId::one().id(3).load().await.unwrap();

    assert!(user.is_some());

    let user = user.unwrap();

    assert_eq!(user.id, 3);

    let user = user.drop().await.unwrap();

    assert_eq!(user.id, 3);

    // Check that it is saved in DB
    let user = User::one().id(3).load().await.unwrap();

    assert!(user.is_none());
}
