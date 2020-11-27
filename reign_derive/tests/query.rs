mod schema;

use reign::{model::diesel, prelude::*};
use serial_test::serial;

#[derive(Debug, Model)]
pub struct User {
    id: i32,
    #[model(tag(details))]
    name: String,
    #[model(tag(email, details))]
    email: Option<String>,
}

#[tokio::test(threaded_scheduler)]
#[serial]
async fn test_all() {
    schema::setup().await;
    let all = User::all().load().await.unwrap();

    assert_eq!(all.len(), 3);
    assert_eq!(all[0].id, 1);
    assert_eq!(all[0].name, "John");
    assert_eq!(all[0].email, None);
    assert_eq!(all[1].id, 2);
    assert_eq!(all[1].name, "Sean");
    assert_eq!(all[1].email, Some("sean@mail.com".into()));
    assert_eq!(all[2].id, 3);
    assert_eq!(all[2].name, "John");
    assert_eq!(all[2].email, Some("john@mail.com".into()));
}

#[tokio::test(threaded_scheduler)]
#[serial]
async fn test_one() {
    schema::setup().await;
    let one = User::one().load().await.unwrap();

    assert!(one.is_some());

    let one = one.unwrap();

    assert_eq!(one.id, 1);
    assert_eq!(one.name, "John");
    assert_eq!(one.email, None);
}

#[tokio::test(threaded_scheduler)]
#[serial]
async fn test_all_filter() {
    schema::setup().await;
    let all = User::all().name("John").load().await.unwrap();

    assert_eq!(all.len(), 2);
    assert_eq!(all[0].id, 1);
    assert_eq!(all[0].name, "John");
    assert_eq!(all[0].email, None);
    assert_eq!(all[1].id, 3);
    assert_eq!(all[1].name, "John");
    assert_eq!(all[1].email, Some("john@mail.com".into()));
}

#[tokio::test(threaded_scheduler)]
#[serial]
async fn test_one_filter() {
    schema::setup().await;
    let one = User::one().name("Sean").load().await.unwrap();

    assert!(one.is_some());

    let one = one.unwrap();

    assert_eq!(one.id, 2);
    assert_eq!(one.name, "Sean");
    assert_eq!(one.email, Some("sean@mail.com".into()));
}

#[tokio::test(threaded_scheduler)]
#[serial]
async fn test_all_limit_offset() {
    schema::setup().await;
    let all = User::all().offset(1).limit(1).load().await.unwrap();

    assert_eq!(all.len(), 1);
    assert_eq!(all[0].id, 2);
    assert_eq!(all[0].name, "Sean");
    assert_eq!(all[0].email, Some("sean@mail.com".into()));
}

#[tokio::test(threaded_scheduler)]
#[serial]
async fn test_tag_all() {
    schema::setup().await;
    let all = UserEmail::all().load().await.unwrap();

    assert_eq!(all.len(), 3);
    assert_eq!(all[0].email, None);
    assert_eq!(all[1].email, Some("sean@mail.com".into()));
    assert_eq!(all[2].email, Some("john@mail.com".into()));
}

#[tokio::test(threaded_scheduler)]
#[serial]
async fn test_tag_one() {
    schema::setup().await;
    let one = UserEmail::one().load().await.unwrap();

    assert!(one.is_some());
    assert_eq!(one.unwrap().email, None);
}

#[tokio::test(threaded_scheduler)]
#[serial]
async fn test_multi_tag_all() {
    schema::setup().await;
    let all = UserDetails::all().load().await.unwrap();

    assert_eq!(all.len(), 3);
    assert_eq!(all[0].name, "John");
    assert_eq!(all[0].email, None);
    assert_eq!(all[1].name, "Sean");
    assert_eq!(all[1].email, Some("sean@mail.com".into()));
    assert_eq!(all[2].name, "John");
    assert_eq!(all[2].email, Some("john@mail.com".into()));
}

#[tokio::test(threaded_scheduler)]
#[serial]
async fn test_multi_tag_one() {
    schema::setup().await;
    let one = UserDetails::one().load().await.unwrap();

    assert!(one.is_some());

    let one = one.unwrap();

    assert_eq!(one.name, "John");
    assert_eq!(one.email, None);
}
