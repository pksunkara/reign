use futures::FutureExt;
use reign_router::{
    hyper::{body::to_bytes, Body, Request as Req, StatusCode},
    service, HandleFuture, Request, Response,
};

#[tokio::test]
async fn test_empty() {
    fn index(_: &mut Request) -> HandleFuture {
        async { Ok("index".respond()?) }.boxed()
    }

    let service = service(|r| {
        r.get("", index);
    });

    let res = service
        .call(
            Req::get("https://reign.rs").body(Body::empty()).unwrap(),
            "10.10.10.10:80".parse().unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(to_bytes(res.into_body()).await.unwrap(), "index");
}

#[tokio::test]
async fn test_trailing_slash() {
    fn index(_: &mut Request) -> HandleFuture {
        async { Ok("index".respond()?) }.boxed()
    }

    let service = service(|r| {
        r.get("index", index);
    });

    let res = service
        .call(
            Req::get("https://reign.rs/index/")
                .body(Body::empty())
                .unwrap(),
            "10.10.10.10:80".parse().unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(to_bytes(res.into_body()).await.unwrap(), "index");
}
