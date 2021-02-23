use reign::router::hyper::{body::to_bytes, Body, Request as Req, StatusCode};

mod common;

#[tokio::test]
async fn test_root_dir() {
    let service = common::service(0);

    let res = service
        .call(
            Req::get("https://reign.rs/static")
                .body(Body::empty())
                .unwrap(),
            "10.10.10.10:80".parse().unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::MOVED_PERMANENTLY);
    assert_eq!(res.headers().get("location").unwrap(), "/static/index.html");
    assert_eq!(to_bytes(res.into_body()).await.unwrap(), "");
}

#[tokio::test]
async fn test_root_dir_trailing_slash() {
    let service = common::service(0);

    let res = service
        .call(
            Req::get("https://reign.rs/static/")
                .body(Body::empty())
                .unwrap(),
            "10.10.10.10:80".parse().unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::MOVED_PERMANENTLY);
    assert_eq!(res.headers().get("location").unwrap(), "/static/index.html");
    assert_eq!(to_bytes(res.into_body()).await.unwrap(), "");
}

#[tokio::test]
async fn test_dir() {
    let service = common::service(0);

    let res = service
        .call(
            Req::get("https://reign.rs/static/css")
                .body(Body::empty())
                .unwrap(),
            "10.10.10.10:80".parse().unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::MOVED_PERMANENTLY);
    assert_eq!(
        res.headers().get("location").unwrap(),
        "/static/css/index.html"
    );
    assert_eq!(to_bytes(res.into_body()).await.unwrap(), "");
}

#[tokio::test]
async fn test_dir_trailing_slash() {
    let service = common::service(0);

    let res = service
        .call(
            Req::get("https://reign.rs/static/css/")
                .body(Body::empty())
                .unwrap(),
            "10.10.10.10:80".parse().unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::MOVED_PERMANENTLY);
    assert_eq!(
        res.headers().get("location").unwrap(),
        "/static/css/index.html"
    );
    assert_eq!(to_bytes(res.into_body()).await.unwrap(), "");
}
