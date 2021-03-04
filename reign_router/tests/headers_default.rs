use reign_router::{
    hyper::{body::to_bytes, Body, Request as Req, StatusCode},
    middleware::HeadersDefault,
    service, Error, Request, Response,
};

#[tokio::test]
async fn test_headers_default() {
    async fn index(_: &mut Request) -> Result<impl Response, Error> {
        Ok("index")
    }

    let service = service(|r| {
        r.pipe("app")
            .add(HeadersDefault::empty().add("x-powered-by", "reign"));

        r.scope("").through(&["app"]).to(|r| {
            r.get("foo", index);
        });
    });

    let res = service
        .clone()
        .call(
            Req::get("https://reign.rs/foo")
                .body(Body::empty())
                .unwrap(),
            "10.10.10.10:80".parse().unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert!(res.headers().contains_key("x-powered-by"));
    assert_eq!(to_bytes(res.into_body()).await.unwrap(), "index");
}
