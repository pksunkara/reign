use reign::router::hyper::{body::to_bytes, Body, Request as Req, StatusCode};

mod common;

#[tokio::test]
async fn test_file() {
    let service = common::service(0);

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
    assert_eq!(res.headers().get("content-type").unwrap(), "text/html");
    assert_eq!(res.headers().get("content-length").unwrap(), "76");
    assert_eq!(
        to_bytes(res.into_body()).await.unwrap(),
        "<html>\n  <head>\n    <title>Hey</title>\n  </head>\n  <body>Hey</body>\n</html>\n"
    );
}

#[tokio::test]
async fn test_mime() {
    let service = common::service(0);

    let res = service
        .call(
            Req::get("https://reign.rs/static/css/app.css")
                .body(Body::empty())
                .unwrap(),
            "10.10.10.10:80".parse().unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.headers().get("content-type").unwrap(), "text/css");
    assert_eq!(res.headers().get("content-length").unwrap(), "25");
    assert_eq!(
        to_bytes(res.into_body()).await.unwrap(),
        "body {\n  color: green;\n}\n"
    );
}
