use hyper::{
    http::Error,
    service::{make_service_fn, service_fn},
    Body, Request, Response, Server,
};
use reign::prelude::*;

#[derive(serde::Serialize)]
struct User {
    name: String,
}

views!("src", "views");

async fn handle(req: Request<Body>) -> Result<Response<Body>, Error> {
    match req.uri().path() {
        "/" => {
            let msg = "Hello Reign!";

            Ok(render!(app)?)
        }
        "/world" => Ok(redirect("/")?),
        "/hey" => {
            let msg = "Hey Reign!";

            Ok(render!(app, status = 404)?)
        }
        "/json" => {
            let user = User {
                name: "Reign".to_string(),
            };

            Ok(json!(user)?)
        }
        "/json_err" => {
            let user = User {
                name: "Reign".to_string(),
            };

            Ok(json!(user, status = 422)?)
        }
        _ => Ok(Response::new(Body::empty())),
    }
}

async fn server() {
    let make_svc = make_service_fn(|_conn| async { Ok::<_, Error>(service_fn(handle)) });

    Server::bind(&([127, 0, 0, 1], 8080).into())
        .serve(make_svc)
        .await
        .unwrap()
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
    use tokio::{select, time::sleep};

    #[tokio::test]
    async fn test_server() {
        let client = async {
            sleep(Duration::from_millis(100)).await;

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
