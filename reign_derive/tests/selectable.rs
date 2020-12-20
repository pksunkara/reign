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

#[derive(Debug, Model)]
#[model(table_name = users)]
pub struct Foo {
    #[model(column_name = id)]
    bar: i32,
    name: String,
}

#[tokio::test(threaded_scheduler)]
#[serial]
async fn test_all() {
    schema::setup().await;

    let all = User::all().await.unwrap();

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

    let one = User::one().await.unwrap();

    assert!(one.is_some());

    let one = one.unwrap();

    assert_eq!(one.id, 1);
    assert_eq!(one.name, "John");
}

#[tokio::test(threaded_scheduler)]
#[serial]
async fn test_rename() {
    schema::setup().await;

    let all = Foo::all().await.unwrap();

    assert_eq!(all.len(), 3);
    assert_eq!(all[0].bar, 1);
    assert_eq!(all[0].name, "John");
    assert_eq!(all[1].bar, 2);
    assert_eq!(all[1].name, "Sean");
    assert_eq!(all[2].bar, 3);
    assert_eq!(all[2].name, "John");
}

#[tokio::test(threaded_scheduler)]
#[serial]
async fn test_all_filter() {
    schema::setup().await;

    let all = User::filter().name("John").all().await.unwrap();

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

    let one = User::filter().name("Sean").one().await.unwrap();

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

    let all = User::filter().all_with(Some(1), Some(1)).await.unwrap();

    assert_eq!(all.len(), 1);
    assert_eq!(all[0].id, 2);
    assert_eq!(all[0].name, "Sean");
    assert_eq!(all[0].email, Some("sean@mail.com".into()));
}

#[tokio::test(threaded_scheduler)]
#[serial]
async fn test_one_offset() {
    schema::setup().await;

    let one = User::filter().one_with(Some(1)).await.unwrap();

    assert!(one.is_some());

    let one = one.unwrap();

    assert_eq!(one.id, 2);
    assert_eq!(one.name, "Sean");
    assert_eq!(one.email, Some("sean@mail.com".into()));
}

#[tokio::test(threaded_scheduler)]
#[serial]
async fn test_tag_all() {
    schema::setup().await;

    let all = UserEmail::all().await.unwrap();

    assert_eq!(all.len(), 3);
    assert_eq!(all[0].email, None);
    assert_eq!(all[1].email, Some("sean@mail.com".into()));
    assert_eq!(all[2].email, Some("john@mail.com".into()));
}

#[tokio::test(threaded_scheduler)]
#[serial]
async fn test_tag_one() {
    schema::setup().await;

    let one = UserEmail::one().await.unwrap();

    assert!(one.is_some());
    assert_eq!(one.unwrap().email, None);
}

#[tokio::test(threaded_scheduler)]
#[serial]
async fn test_multi_tag_all() {
    schema::setup().await;

    let all = UserDetails::all().await.unwrap();

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

    let one = UserDetails::one().await.unwrap();

    assert!(one.is_some());

    let one = one.unwrap();

    assert_eq!(one.name, "John");
    assert_eq!(one.email, None);
}
