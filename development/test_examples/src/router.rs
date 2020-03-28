pub use reqwest::{Client, StatusCode};

pub async fn test(bad_method_status: StatusCode) {
    let mut url;
    let client = Client::new();

    url = "http://localhost:8080/str";

    let res = client.get(url).send().await.unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert!(res.headers().contains_key("x-powered-by"));
    assert!(res.headers().contains_key("x-content-type-options"));
    assert_eq!(res.text().await.unwrap(), "str");

    let res = client.post(url).send().await.unwrap();

    assert_eq!(res.status(), bad_method_status);

    url = "http://localhost:8080/string";

    let res = client.get(url).send().await.unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert!(res.headers().contains_key("x-powered-by"));
    assert!(res.headers().contains_key("x-content-type-options"));
    assert_eq!(res.text().await.unwrap(), "string");

    url = "http://localhost:8080/response";

    let res = client.get(url).send().await.unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert!(res.headers().contains_key("x-powered-by"));
    assert!(res.headers().contains_key("x-content-type-options"));
    assert_eq!(res.text().await.unwrap(), "response");

    url = "http://localhost:8080/error";

    let res = client.get(url).send().await.unwrap();

    assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(res.text().await.unwrap(), "");

    url = "http://localhost:8080/post";

    let res = client.post(url).send().await.unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert!(res.headers().contains_key("x-powered-by"));
    assert!(res.headers().contains_key("x-content-type-options"));
    assert_eq!(res.text().await.unwrap(), "post");

    let res = client.get(url).send().await.unwrap();

    assert_eq!(res.status(), bad_method_status);

    url = "http://localhost:8080/put";

    let res = client.put(url).send().await.unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert!(res.headers().contains_key("x-powered-by"));
    assert!(res.headers().contains_key("x-content-type-options"));
    assert_eq!(res.text().await.unwrap(), "put");

    let res = client.get(url).send().await.unwrap();

    assert_eq!(res.status(), bad_method_status);

    url = "http://localhost:8080/patch";

    let res = client.patch(url).send().await.unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert!(res.headers().contains_key("x-powered-by"));
    assert!(res.headers().contains_key("x-content-type-options"));
    assert_eq!(res.text().await.unwrap(), "patch");

    let res = client.get(url).send().await.unwrap();

    assert_eq!(res.status(), bad_method_status);

    url = "http://localhost:8080/delete";

    let res = client.delete(url).send().await.unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert!(res.headers().contains_key("x-powered-by"));
    assert!(res.headers().contains_key("x-content-type-options"));
    assert_eq!(res.text().await.unwrap(), "delete");

    let res = client.get(url).send().await.unwrap();

    assert_eq!(res.status(), bad_method_status);

    url = "http://localhost:8080/methods";

    let res = client.post(url).send().await.unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert!(res.headers().contains_key("x-powered-by"));
    assert!(res.headers().contains_key("x-content-type-options"));
    assert_eq!(res.text().await.unwrap(), "methods");

    let res = client.put(url).send().await.unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert!(res.headers().contains_key("x-powered-by"));
    assert!(res.headers().contains_key("x-content-type-options"));
    assert_eq!(res.text().await.unwrap(), "methods");

    let res = client.get(url).send().await.unwrap();

    assert_eq!(res.status(), bad_method_status);
}
