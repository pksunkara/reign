use futures::FutureExt;
use reign_router::{
    hyper::{body::to_bytes, Body, Method, Request as Req, StatusCode},
    service, HandleFuture, Request, Response,
};

macro_rules! call {
    ($service:ident, $path:expr, $method:ident) => {
        let res = $service
            .clone()
            .call(
                Req::$method($path)
                    .body(Body::empty())
                    .unwrap(),
                "10.10.10.10:80".parse().unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::NOT_FOUND);
        assert_eq!(to_bytes(res.into_body()).await.unwrap(), "");
    };
    ($service:ident, $path:expr, $method:ident, $($others:ident),+) => {
        call!($service, $path, $method);
        call!($service, $path, $($others),+);
    }
}

#[tokio::test]
async fn test_method_get() {
    fn get(_: &mut Request) -> HandleFuture {
        async { Ok("get".respond()?) }.boxed()
    }

    let service = service(|r| {
        r.get("get", get);
    });

    call!(
        service,
        "https://reign.rs/get",
        post,
        put,
        patch,
        delete,
        head,
        options,
        trace,
        connect
    );

    let res = service
        .call(
            Req::get("https://reign.rs/get")
                .body(Body::empty())
                .unwrap(),
            "10.10.10.10:80".parse().unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(to_bytes(res.into_body()).await.unwrap(), "get");
}

#[tokio::test]
async fn test_method_post() {
    fn post(_: &mut Request) -> HandleFuture {
        async { Ok("post".respond()?) }.boxed()
    }

    let service = service(|r| {
        r.post("post", post);
    });

    call!(
        service,
        "https://reign.rs/post",
        get,
        put,
        patch,
        delete,
        head,
        options,
        trace,
        connect
    );

    let res = service
        .call(
            Req::post("https://reign.rs/post")
                .body(Body::empty())
                .unwrap(),
            "10.10.10.10:80".parse().unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(to_bytes(res.into_body()).await.unwrap(), "post");
}

#[tokio::test]
async fn test_method_put() {
    fn put(_: &mut Request) -> HandleFuture {
        async { Ok("put".respond()?) }.boxed()
    }

    let service = service(|r| {
        r.put("put", put);
    });

    call!(
        service,
        "https://reign.rs/put",
        post,
        get,
        patch,
        delete,
        head,
        options,
        trace,
        connect
    );

    let res = service
        .call(
            Req::put("https://reign.rs/put")
                .body(Body::empty())
                .unwrap(),
            "10.10.10.10:80".parse().unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(to_bytes(res.into_body()).await.unwrap(), "put");
}

#[tokio::test]
async fn test_method_patch() {
    fn patch(_: &mut Request) -> HandleFuture {
        async { Ok("patch".respond()?) }.boxed()
    }

    let service = service(|r| {
        r.patch("patch", patch);
    });

    call!(
        service,
        "https://reign.rs/patch",
        post,
        put,
        get,
        delete,
        head,
        options,
        trace,
        connect
    );

    let res = service
        .call(
            Req::patch("https://reign.rs/patch")
                .body(Body::empty())
                .unwrap(),
            "10.10.10.10:80".parse().unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(to_bytes(res.into_body()).await.unwrap(), "patch");
}

#[tokio::test]
async fn test_method_delete() {
    fn delete(_: &mut Request) -> HandleFuture {
        async { Ok("delete".respond()?) }.boxed()
    }

    let service = service(|r| {
        r.delete("delete", delete);
    });

    call!(
        service,
        "https://reign.rs/delete",
        post,
        put,
        patch,
        get,
        head,
        options,
        trace,
        connect
    );

    let res = service
        .call(
            Req::delete("https://reign.rs/delete")
                .body(Body::empty())
                .unwrap(),
            "10.10.10.10:80".parse().unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(to_bytes(res.into_body()).await.unwrap(), "delete");
}

#[tokio::test]
async fn test_method_head() {
    fn head(_: &mut Request) -> HandleFuture {
        async { Ok("head".respond()?) }.boxed()
    }

    let service = service(|r| {
        r.head("head", head);
    });

    call!(
        service,
        "https://reign.rs/head",
        post,
        put,
        patch,
        delete,
        get,
        options,
        trace,
        connect
    );

    let res = service
        .call(
            Req::head("https://reign.rs/head")
                .body(Body::empty())
                .unwrap(),
            "10.10.10.10:80".parse().unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(to_bytes(res.into_body()).await.unwrap(), "head");
}

#[tokio::test]
async fn test_method_options() {
    fn options(_: &mut Request) -> HandleFuture {
        async { Ok("options".respond()?) }.boxed()
    }

    let service = service(|r| {
        r.options("options", options);
    });

    call!(
        service,
        "https://reign.rs/options",
        post,
        put,
        patch,
        delete,
        head,
        get,
        trace,
        connect
    );

    let res = service
        .call(
            Req::options("https://reign.rs/options")
                .body(Body::empty())
                .unwrap(),
            "10.10.10.10:80".parse().unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(to_bytes(res.into_body()).await.unwrap(), "options");
}

#[tokio::test]
async fn test_method_trace() {
    fn trace(_: &mut Request) -> HandleFuture {
        async { Ok("trace".respond()?) }.boxed()
    }

    let service = service(|r| {
        r.trace("trace", trace);
    });

    call!(
        service,
        "https://reign.rs/trace",
        post,
        put,
        patch,
        delete,
        head,
        options,
        get,
        connect
    );

    let res = service
        .call(
            Req::trace("https://reign.rs/trace")
                .body(Body::empty())
                .unwrap(),
            "10.10.10.10:80".parse().unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(to_bytes(res.into_body()).await.unwrap(), "trace");
}

#[tokio::test]
async fn test_method_connect() {
    fn connect(_: &mut Request) -> HandleFuture {
        async { Ok("connect".respond()?) }.boxed()
    }

    let service = service(|r| {
        r.connect("connect", connect);
    });

    call!(
        service,
        "https://reign.rs/connect",
        post,
        put,
        patch,
        delete,
        head,
        options,
        trace,
        get
    );

    let res = service
        .call(
            Req::connect("https://reign.rs/connect")
                .body(Body::empty())
                .unwrap(),
            "10.10.10.10:80".parse().unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(to_bytes(res.into_body()).await.unwrap(), "connect");
}

#[tokio::test]
async fn test_mutliple_methods() {
    fn index(_: &mut Request) -> HandleFuture {
        async { Ok("index".respond()?) }.boxed()
    }

    let service = service(|r| {
        r.any(&[Method::GET, Method::POST], "index", index);
    });

    call!(
        service,
        "https://reign.rs/index",
        put,
        patch,
        delete,
        head,
        options,
        trace,
        connect
    );

    let res = service
        .clone()
        .call(
            Req::get("https://reign.rs/index")
                .body(Body::empty())
                .unwrap(),
            "10.10.10.10:80".parse().unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(to_bytes(res.into_body()).await.unwrap(), "index");

    let res = service
        .call(
            Req::post("https://reign.rs/index")
                .body(Body::empty())
                .unwrap(),
            "10.10.10.10:80".parse().unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(to_bytes(res.into_body()).await.unwrap(), "index");
}
