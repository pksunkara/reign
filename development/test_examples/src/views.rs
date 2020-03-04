use reqwest::{redirect::Policy, Client, StatusCode};

pub async fn test() {
    let client = Client::builder().redirect(Policy::none()).build().unwrap();

    let response = client.get("http://localhost:8080").send().await.unwrap();

    assert_eq!(
        response.text().await.unwrap(),
        "<div>\n  <p>Hello World!</p>\n</div>"
    );

    let response = client
        .get("http://localhost:8080/world")
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::SEE_OTHER);

    let response = client
        .get("http://localhost:8080/hey")
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    assert_eq!(
        response.text().await.unwrap(),
        "<div>\n  <p>Hey!</p>\n</div>"
    );
}
