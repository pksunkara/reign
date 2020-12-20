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
async fn test_filter_set() {
    schema::setup().await;

    let changes = User::filter()
        .name("John")
        .set()
        .name("Mike".into())
        .save()
        .await
        .unwrap();

    assert_eq!(changes.len(), 2);
    assert_eq!(changes[0].id, 1);
    assert_eq!(changes[0].name, "Mike");
    assert_eq!(changes[0].email, None);
    assert_eq!(changes[1].id, 3);
    assert_eq!(changes[1].name, "Mike");
    assert_eq!(changes[1].email, Some("john@mail.com".into()));

    // Check that it is saved in DB
    let all = User::all().await.unwrap();

    assert_eq!(all.len(), 3);
    assert_eq!(all[0].id, 2);
    assert_eq!(all[0].name, "Sean");
    assert_eq!(all[0].email, Some("sean@mail.com".into()));
    assert_eq!(all[1].id, 1);
    assert_eq!(all[1].name, "Mike");
    assert_eq!(all[1].email, None);
    assert_eq!(all[2].id, 3);
    assert_eq!(all[2].name, "Mike");
    assert_eq!(all[2].email, Some("john@mail.com".into()));
}

#[tokio::test(threaded_scheduler)]
#[serial]
async fn test_set() {
    schema::setup().await;

    let user = User::filter().id(3).one().await.unwrap();

    assert!(user.is_some());

    let user = user.unwrap();

    assert_eq!(user.id, 3);
    assert_eq!(user.name, "John");
    assert_eq!(user.email, Some("john@mail.com".into()));

    let user = user.set().name("Mike".into()).save().await.unwrap();

    assert_eq!(user.id, 3);
    assert_eq!(user.name, "Mike");
    assert_eq!(user.email, Some("john@mail.com".into()));

    // Check that it is saved in DB
    let user = User::filter().id(3).one().await.unwrap();

    assert!(user.is_some());

    let user = user.unwrap();

    assert_eq!(user.id, 3);
    assert_eq!(user.name, "Mike");
    assert_eq!(user.email, Some("john@mail.com".into()));
}

#[tokio::test(threaded_scheduler)]
#[serial]
async fn test_set_null() {
    schema::setup().await;

    let user = User::filter().id(3).one().await.unwrap();

    assert!(user.is_some());

    let user = user.unwrap();

    assert_eq!(user.id, 3);
    assert_eq!(user.name, "John");
    assert_eq!(user.email, Some("john@mail.com".into()));

    let user = user.set().email(None).save().await.unwrap();

    assert_eq!(user.id, 3);
    assert_eq!(user.name, "John");
    assert_eq!(user.email, None);

    // Check that it is saved in DB
    let user = User::filter().id(3).one().await.unwrap();

    assert!(user.is_some());

    let user = user.unwrap();

    assert_eq!(user.id, 3);
    assert_eq!(user.name, "John");
    assert_eq!(user.email, None);
}

#[tokio::test(threaded_scheduler)]
#[serial]
async fn test_tag_filter_set() {
    schema::setup().await;

    let changes = UserId::filter()
        .name("John")
        .set()
        .name("Mike".into())
        .save()
        .await
        .unwrap();

    assert_eq!(changes.len(), 2);
    assert_eq!(changes[0].id, 1);
    assert_eq!(changes[1].id, 3);

    // Check that it is saved in DB
    let all = User::all().await.unwrap();

    assert_eq!(all.len(), 3);
    assert_eq!(all[0].id, 2);
    assert_eq!(all[0].name, "Sean");
    assert_eq!(all[0].email, Some("sean@mail.com".into()));
    assert_eq!(all[1].id, 1);
    assert_eq!(all[1].name, "Mike");
    assert_eq!(all[1].email, None);
    assert_eq!(all[2].id, 3);
    assert_eq!(all[2].name, "Mike");
    assert_eq!(all[2].email, Some("john@mail.com".into()));
}

#[tokio::test(threaded_scheduler)]
#[serial]
async fn test_tag_set() {
    schema::setup().await;

    let user = UserId::filter().id(3).one().await.unwrap();

    assert!(user.is_some());

    let user = user.unwrap();

    assert_eq!(user.id, 3);

    let user = user.set().name("Mike".into()).save().await.unwrap();

    assert_eq!(user.id, 3);

    // Check that it is saved in DB
    let user = User::filter().id(3).one().await.unwrap();

    assert!(user.is_some());

    let user = user.unwrap();

    assert_eq!(user.id, 3);
    assert_eq!(user.name, "Mike");
    assert_eq!(user.email, Some("john@mail.com".into()));
}
