use reign::{
    prelude::*,
    router::router::{
        hyper::{Body, Response},
        serve, Error, Request, Router,
    },
};

#[derive(serde::Serialize)]
struct User {
    name: String,
}

views!("src", "views");

async fn index(_: Request) -> Result<Response<Body>, Error> {
    let msg = "Hello Reign!";

    Ok(render!(app)?)
}

async fn world(_: Request) -> Result<Response<Body>, Error> {
    Ok(redirect("/")?)
}

async fn hey(_: Request) -> Result<Response<Body>, Error> {
    let msg = "Hey Reign!";

    Ok(render!(app, status = 404)?)
}

async fn json(_: Request) -> Result<Response<Body>, Error> {
    let user = User {
        name: "Reign".to_string(),
    };

    Ok(json!(user)?)
}

async fn json_err(_: Request) -> Result<Response<Body>, Error> {
    let user = User {
        name: "Reign".to_string(),
    };

    Ok(json!(user, status = 422)?)
}

fn router(r: &mut Router) {
    r.get("", index);
    r.get("world", world);
    r.get("hey", hey);
    r.get("json", json);
    r.get("json_err", json_err);
}

async fn server() {
    serve("127.0.0.1:8080", router).await.unwrap()
}

#[tokio::main]
async fn main() {
    server().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::{redirect::Policy, Client, StatusCode};
    use std::time::Duration;
    use tokio::{select, time::delay_for};

    #[tokio::test]
    async fn test_server() {
        let client = async {
            delay_for(Duration::from_millis(100)).await;

            let client = Client::builder().redirect(Policy::none()).build().unwrap();

            let response = client.get("http://localhost:8080").send().await.unwrap();

            assert_eq!(response.status(), StatusCode::OK);
            assert_eq!(
                response.text().await.unwrap(),
                "<div>\n  <p>Hello Reign!</p>\n</div>"
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
                "<div>\n  <p>Hey Reign!</p>\n</div>"
            );

            let response = client
                .get("http://localhost:8080/json")
                .send()
                .await
                .unwrap();

            assert_eq!(response.status(), StatusCode::OK);
            assert_eq!(response.text().await.unwrap(), "{\"name\":\"Reign\"}");

            let response = client
                .get("http://localhost:8080/json_err")
                .send()
                .await
                .unwrap();

            assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
            assert_eq!(response.text().await.unwrap(), "{\"name\":\"Reign\"}");
        };

        select! {
            _ =  server() => {}
            _ = client => {}
        }
    }
}
