use reqwest::{redirect::Policy, Client, StatusCode};

pub async fn test(router: &str) {
    let client = Client::builder().redirect(Policy::none()).build().unwrap();

    let response = client.get("http://localhost:8080").send().await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.text().await.unwrap(),
        format!("<div>\n  <p>Hello {}!</p>\n</div>", router)
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
        format!("<div>\n  <p>Hey {}!</p>\n</div>", router)
    );

    let response = client
        .get("http://localhost:8080/json")
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response.text().await.unwrap(),
        format!("{{\"name\":\"{}\"}}", router)
    );

    let response = client
        .get("http://localhost:8080/json_err")
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    assert_eq!(
        response.text().await.unwrap(),
        format!("{{\"name\":\"{}\"}}", router)
    );
}