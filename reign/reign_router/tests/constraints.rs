use reign_router::router::{
    hyper::{body::to_bytes, Body, Method, Request as Req, Response as Res, StatusCode},
    service, Error, Request, Response,
};

#[tokio::test]
async fn test_constraint() {
    async fn index(_: &mut Request) -> Result<Res<Body>, Error> {
        Ok("index".respond()?)
    }

    let service = service(|r| {
        r.any_with_constraint(
            &[Method::GET],
            "foo",
            |req| req.uri().port().is_some(),
            index,
        );
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

    assert_eq!(res.status(), StatusCode::NOT_FOUND);
    assert_eq!(to_bytes(res.into_body()).await.unwrap(), "");

    let res = service
        .call(
            Req::get("https://reign.rs:8080/foo")
                .body(Body::empty())
                .unwrap(),
            "10.10.10.10:80".parse().unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(to_bytes(res.into_body()).await.unwrap(), "index");
}
