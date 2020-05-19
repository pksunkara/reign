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

    url = "http://localhost:8080/multi_methods";

    let res = client.post(url).send().await.unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert!(res.headers().contains_key("x-powered-by"));
    assert!(res.headers().contains_key("x-content-type-options"));
    assert_eq!(res.text().await.unwrap(), "multi_methods");

    let res = client.put(url).send().await.unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert!(res.headers().contains_key("x-powered-by"));
    assert!(res.headers().contains_key("x-content-type-options"));
    assert_eq!(res.text().await.unwrap(), "multi_methods");

    let res = client.get(url).send().await.unwrap();

    assert_eq!(res.status(), bad_method_status);

    url = "http://localhost:8080/scope_static";

    let res = client.get(url).send().await.unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert!(res.headers().contains_key("x-powered-by"));
    assert!(res.headers().contains_key("x-content-type-options"));
    assert_eq!(res.text().await.unwrap(), "scope_static");

    url = "http://localhost:8080/pipe";

    let res = client.get(url).send().await.unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert!(res.headers().contains_key("x-powered-by"));
    assert!(res.headers().contains_key("x-content-type-options"));
    assert!(res.headers().contains_key("x-runtime"));
    assert_eq!(res.text().await.unwrap(), "pipe");

    url = "http://localhost:8080/pipe_empty";

    let res = client.get(url).send().await.unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert!(res.headers().contains_key("x-powered-by"));
    assert!(res.headers().contains_key("x-content-type-options"));
    assert_eq!(res.text().await.unwrap(), "pipe_empty");

    url = "http://localhost:8080/param/foobar";

    let res = client.get(url).send().await.unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert!(res.headers().contains_key("x-powered-by"));
    assert!(res.headers().contains_key("x-content-type-options"));
    assert_eq!(res.text().await.unwrap(), "param foobar");

    url = "http://localhost:8080/param_optional/foobar";

    let res = client.get(url).send().await.unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert!(res.headers().contains_key("x-powered-by"));
    assert!(res.headers().contains_key("x-content-type-options"));
    assert_eq!(res.text().await.unwrap(), "param_optional foobar");

    url = "http://localhost:8080/param_optional";

    let res = client.get(url).send().await.unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert!(res.headers().contains_key("x-powered-by"));
    assert!(res.headers().contains_key("x-content-type-options"));
    assert_eq!(res.text().await.unwrap(), "param_optional ");

    url = "http://localhost:8080/param_regex/123";

    let res = client.get(url).send().await.unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert!(res.headers().contains_key("x-powered-by"));
    assert!(res.headers().contains_key("x-content-type-options"));
    assert_eq!(res.text().await.unwrap(), "param_regex 123");

    url = "http://localhost:8080/param_regex/foobar";

    let res = client.get(url).send().await.unwrap();

    assert_eq!(res.status(), StatusCode::NOT_FOUND);
    assert_eq!(res.text().await.unwrap(), "");

    url = "http://localhost:8080/param_optional_regex/123";

    let res = client.get(url).send().await.unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert!(res.headers().contains_key("x-powered-by"));
    assert!(res.headers().contains_key("x-content-type-options"));
    assert_eq!(res.text().await.unwrap(), "param_optional_regex 123");

    url = "http://localhost:8080/param_optional_regex";

    let res = client.get(url).send().await.unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert!(res.headers().contains_key("x-powered-by"));
    assert!(res.headers().contains_key("x-content-type-options"));
    assert_eq!(res.text().await.unwrap(), "param_optional_regex ");

    url = "http://localhost:8080/param_optional_regex/foobar";

    let res = client.get(url).send().await.unwrap();

    assert_eq!(res.status(), StatusCode::NOT_FOUND);
    assert_eq!(res.text().await.unwrap(), "");

    url = "http://localhost:8080/scope_param/foobar/bar";

    let res = client.get(url).send().await.unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert!(res.headers().contains_key("x-powered-by"));
    assert!(res.headers().contains_key("x-content-type-options"));
    assert_eq!(res.text().await.unwrap(), "scope_param foobar");

    url = "http://localhost:8080/scope_param_optional/foobar/bar";

    let res = client.get(url).send().await.unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert!(res.headers().contains_key("x-powered-by"));
    assert!(res.headers().contains_key("x-content-type-options"));
    assert_eq!(res.text().await.unwrap(), "scope_param_optional foobar");

    url = "http://localhost:8080/scope_param_optional/bar";

    let res = client.get(url).send().await.unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert!(res.headers().contains_key("x-powered-by"));
    assert!(res.headers().contains_key("x-content-type-options"));
    assert_eq!(res.text().await.unwrap(), "scope_param_optional ");

    url = "http://localhost:8080/scope_param_regex/123/bar";

    let res = client.get(url).send().await.unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert!(res.headers().contains_key("x-powered-by"));
    assert!(res.headers().contains_key("x-content-type-options"));
    assert_eq!(res.text().await.unwrap(), "scope_param_regex 123");

    url = "http://localhost:8080/scope_param_regex/foobar/bar";

    let res = client.get(url).send().await.unwrap();

    assert_eq!(res.status(), StatusCode::NOT_FOUND);
    assert_eq!(res.text().await.unwrap(), "");

    url = "http://localhost:8080/scope_param_optional_regex/123/bar";

    let res = client.get(url).send().await.unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert!(res.headers().contains_key("x-powered-by"));
    assert!(res.headers().contains_key("x-content-type-options"));
    assert_eq!(res.text().await.unwrap(), "scope_param_optional_regex 123");

    url = "http://localhost:8080/scope_param_optional_regex/bar";

    let res = client.get(url).send().await.unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert!(res.headers().contains_key("x-powered-by"));
    assert!(res.headers().contains_key("x-content-type-options"));
    assert_eq!(res.text().await.unwrap(), "scope_param_optional_regex ");

    url = "http://localhost:8080/scope_param_optional_regex/foobar/bar";

    let res = client.get(url).send().await.unwrap();

    assert_eq!(res.status(), StatusCode::NOT_FOUND);
    assert_eq!(res.text().await.unwrap(), "");

    url = "http://localhost:8080/nested_scope/123/foo/456/bar";

    let res = client.get(url).send().await.unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert!(res.headers().contains_key("x-powered-by"));
    assert!(res.headers().contains_key("x-content-type-options"));
    assert_eq!(res.text().await.unwrap(), "nested_scope 123 456");

    url = "http://localhost:8080/multi_params/123/foo/456";

    let res = client.get(url).send().await.unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert!(res.headers().contains_key("x-powered-by"));
    assert!(res.headers().contains_key("x-content-type-options"));
    assert_eq!(res.text().await.unwrap(), "multi_params 123 456");
}
