use reqwest::{header::CONTENT_TYPE, Client, StatusCode};

pub async fn test() {
    let mut url;
    let client = Client::new();

    url = "http://localhost:8080";

    let res = client.post(url).send().await.unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert!(res.headers().contains_key("x-powered-by"));
    assert_eq!(res.text().await.unwrap(), "root");

    let res = client
        .post(url)
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
    assert_eq!(res.text().await.unwrap(), "");

    url = "http://localhost:8080/error";

    let res = client.get(url).send().await.unwrap();

    assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(res.text().await.unwrap(), "");

    url = "http://localhost:8080/account";

    let res = client.get(url).send().await.unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert!(res.headers().contains_key("x-powered-by"));
    assert_eq!(res.text().await.unwrap(), "account");

    url = "http://localhost:8080/orgs";

    let res = client.get(url).send().await.unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert!(res.headers().contains_key("x-powered-by"));
    assert_eq!(res.text().await.unwrap(), "orgs");

    url = "http://localhost:8080/orgs/repos";

    let res = client.get(url).send().await.unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert!(res.headers().contains_key("x-powered-by"));
    assert_eq!(res.text().await.unwrap(), "repos");

    url = "http://localhost:8080/users";

    let res = client.get(url).send().await.unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert!(res.headers().contains_key("x-powered-by"));
    assert!(res.headers().contains_key("x-runtime"));
    assert_eq!(res.text().await.unwrap(), "users");

    url = "http://localhost:8080/api";

    let res = client.get(url).send().await.unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert!(res.headers().contains_key("x-version"));
    assert_eq!(res.text().await.unwrap(), "api");
}
