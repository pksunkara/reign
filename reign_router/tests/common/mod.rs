use reqwest::{header::CONTENT_TYPE, Client, StatusCode};
use std::time::Duration;
use tokio::time::delay_for;

#[allow(dead_code)]
pub async fn content_type_test() {
    let url = "http://localhost:8080";

    delay_for(Duration::from_millis(100)).await;
    let client = Client::new();

    let res = client.post(url).send().await.unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.text().await.unwrap(), "hello");

    let res = client
        .post(url)
        .header(CONTENT_TYPE, "")
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
    assert_eq!(res.text().await.unwrap(), "");

    let res = client
        .post(url)
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.text().await.unwrap(), "hello");

    let res = client
        .post(url)
        .header(CONTENT_TYPE, "application/hal+json")
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert_eq!(res.text().await.unwrap(), "hello");

    let res = client
        .post(url)
        .header(CONTENT_TYPE, "a")
        .send()
        .await
        .unwrap();

    assert_eq!(res.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
    assert_eq!(res.text().await.unwrap(), "");
}

#[allow(dead_code)]
pub async fn runtime_test() {
    let url = "http://localhost:8080";

    delay_for(Duration::from_millis(100)).await;
    let client = Client::new();

    let res = client.get(url).send().await.unwrap();

    assert_eq!(res.status(), StatusCode::OK);
    assert!(res.headers().contains_key("x-runtime"));
    assert_eq!(res.text().await.unwrap(), "hello");
}
