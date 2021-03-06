use reign_router::{
    hyper::{body::to_bytes, Body, Request as Req, StatusCode},
    middleware::HeadersDefault,
    service, Error, Request, Response,
};

async fn index(_: &mut Request) -> Result<impl Response, Error> {
    Ok("index")
}

#[tokio::test]
#[should_panic(expected = "can't find pipe with name `app`")]
async fn test_invalid_pipe() {
    service(|r| {
        r.scope("").through(&["app"]).to(|r| {
            r.get("", index);
        });
    });
}

#[tokio::test]
#[should_panic(expected = "can't find pipe with name `secret`")]
async fn test_scope_pipe_not_visible() {
    service(|r| {
        r.scope("pipe").to(|r| {
            r.pipe("secret")
                .add(HeadersDefault::empty().add("x-powered-by", "reign"));
        });

        r.scope("").through(&["secret"]).to(|r| {
            r.get("", index);
        });
    });
}

#[tokio::test]
async fn test_pipe_in_scope() {
    let service = service(|r| {
        r.scope("pipe").to(|r| {
            r.pipe("secret")
                .add(HeadersDefault::empty().add("x-powered-by", "reign"));

            r.scope("").through(&["secret"]).to(|r| {
                r.get("", index);
            });
        });

        r.scope("").to(|r| {
            r.get("", index);
        });
    });

    let res = service
        .clone()
        .call(
            Req::get("https://reign.rs/pipe")
                .body(Body::empty())
                .unwrap(),
            "10.10.10.10:80".parse().unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert!(res.headers().contains_key("x-powered-by"));
    assert_eq!(to_bytes(res.into_body()).await.unwrap(), "index");

    let res = service
        .clone()
        .call(
            Req::get("https://reign.rs").body(Body::empty()).unwrap(),
            "10.10.10.10:80".parse().unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert!(!res.headers().contains_key("x-powered-by"));
    assert_eq!(to_bytes(res.into_body()).await.unwrap(), "index");
}

#[tokio::test]
async fn test_pipe_in_upper_scope() {
    let service = service(|r| {
        r.pipe("secret")
            .add(HeadersDefault::empty().add("x-powered-by", "reign"));

        r.scope("pipe").to(|r| {
            r.scope("").through(&["secret"]).to(|r| {
                r.get("", index);
            });
        });
    });

    let res = service
        .clone()
        .call(
            Req::get("https://reign.rs/pipe")
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
