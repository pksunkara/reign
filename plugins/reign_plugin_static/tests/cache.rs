use chrono::Utc;
use hyper::header::IF_NONE_MATCH;
use reign::router::hyper::{header::IF_MODIFIED_SINCE, Body, Request as Req, StatusCode};
use tokio::{
    fs::{read_to_string, write},
    time::sleep,
};

use std::{path::PathBuf, time::Duration};

mod common;

#[tokio::test]
async fn test_cache() {
    let service = common::service(300);

    let res = service
        .call(
            Req::get("https://reign.rs/static/index.html")
                .body(Body::empty())
                .unwrap(),
            "10.10.10.10:80".parse().unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(
        res.headers().get("cache-control").unwrap(),
        "public, max-age=300"
    );
    assert!(res.headers().contains_key("last-modified"));
    assert!(res.headers().contains_key("etag"));
}

#[tokio::test]
async fn test_if_modified_since() {
    let service = common::service(300);

    let res = service
        .call(
            Req::get("https://reign.rs/static/index.html")
                .header(IF_MODIFIED_SINCE, Utc::now().to_rfc2822())
                .body(Body::empty())
                .unwrap(),
            "10.10.10.10:80".parse().unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::NOT_MODIFIED);
    assert_eq!(
        res.headers().get("cache-control").unwrap(),
        "public, max-age=300"
    );
    assert!(res.headers().contains_key("last-modified"));
    assert!(res.headers().contains_key("etag"));
}

#[tokio::test]
async fn test_if_none_match() {
    let service = common::service(300);

    let res = service
        .clone()
        .call(
            Req::get("https://reign.rs/static/index.html")
                .body(Body::empty())
                .unwrap(),
            "10.10.10.10:80".parse().unwrap(),
        )
        .await
        .unwrap();

    let etag = res.headers().get("etag").unwrap().to_str().unwrap();

    let res = service
        .call(
            Req::get("https://reign.rs/static/index.html")
                .header(IF_NONE_MATCH, format!("{}, \"8273j\"", etag))
                .body(Body::empty())
                .unwrap(),
            "10.10.10.10:80".parse().unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::NOT_MODIFIED);
    assert_eq!(
        res.headers().get("cache-control").unwrap(),
        "public, max-age=300"
    );
    assert!(res.headers().contains_key("last-modified"));
    assert_eq!(res.headers().get("etag").unwrap(), etag);
}

#[tokio::test]
async fn test_if_none_match_when_modified() {
    let service = common::service(0);

    let res = service
        .call(
            Req::get("https://reign.rs/static/index.html")
                .header(IF_NONE_MATCH, "\"8273j\"")
                .body(Body::empty())
                .unwrap(),
            "10.10.10.10:80".parse().unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert!(res.headers().contains_key("last-modified"));
    assert!(res.headers().contains_key("etag"));
}

#[tokio::test]
async fn test_if_modified_since_when_modified() {
    let service = common::service(0);
    let time = Utc::now().to_rfc2822();

    sleep(Duration::from_secs(1)).await;
    let html = PathBuf::from("tests/fixture/index.html");
    let content = read_to_string(html.clone()).await.unwrap();
    write(html, content).await.unwrap();

    let res = service
        .call(
            Req::get("https://reign.rs/static/index.html")
                .header(IF_MODIFIED_SINCE, time)
                .body(Body::empty())
                .unwrap(),
            "10.10.10.10:80".parse().unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert!(res.headers().contains_key("last-modified"));
    assert!(res.headers().contains_key("etag"));
}
