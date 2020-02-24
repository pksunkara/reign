#[cfg(feature = "router-gotham")]
use gotham::{
    handler::HandlerFuture,
    helpers::http::response::create_empty_response,
    hyper::{header, HeaderMap, StatusCode},
    state::{FromState, State},
};
use mime::{Mime, Name, FORM_DATA, JSON, WWW_FORM_URLENCODED};

#[derive(Clone)]
pub struct ContentType<'a> {
    subtypes: Vec<&'a str>,
}

impl<'a> ContentType<'a> {
    pub fn new(subtypes: Vec<&'a str>) -> Self {
        ContentType { subtypes }
    }

    pub fn default() -> Self {
        ContentType::new(vec![JSON.as_str(), FORM_DATA.as_str()])
    }

    pub fn multipart(mut self) -> Self {
        self.subtypes.push(WWW_FORM_URLENCODED.as_str());
        self
    }

    fn allow(&self, val: Name) -> bool {
        self.subtypes.iter().find(|&&x| x == val.as_str()).is_some()
    }
}

#[cfg(feature = "router-gotham")]
impl<'a> gotham::middleware::Middleware for ContentType<'a> {
    fn call<Chain>(self, state: State, chain: Chain) -> std::pin::Pin<Box<HandlerFuture>>
    where
        Chain: FnOnce(State) -> std::pin::Pin<Box<HandlerFuture>> + Send + 'static,
    {
        use futures::prelude::*;

        match HeaderMap::borrow_from(&state).get(header::CONTENT_TYPE) {
            Some(content_type) => {
                if let Ok(content_type) = content_type.to_str() {
                    if let Ok(val) = content_type.parse::<Mime>() {
                        if self.allow(val.subtype()) {
                            return chain(state);
                        }

                        if let Some(suffix) = val.suffix() {
                            if self.allow(suffix) {
                                return chain(state);
                            }
                        }
                    }
                }
            }
            None => {
                return chain(state);
            }
        };

        let response = create_empty_response(&state, StatusCode::UNSUPPORTED_MEDIA_TYPE);
        future::ok((state, response)).boxed()
    }
}

#[cfg(feature = "router-gotham")]
impl<'a> gotham::middleware::NewMiddleware for ContentType<'a> {
    type Instance = Self;

    fn new_middleware(&self) -> std::io::Result<Self::Instance> {
        Ok(self.clone())
    }
}

// #[cfg(feature = "router-tide")]
// // impl<'a> tide::middleware::Middleware for ContentType<'a> {
// //     fn handle<'b>(&'b self, ctx: Request<State>, next: Next<'b, State>) -> BoxFuture<'b, Response> {
// //         Box::pin(async move {})
// //     }
// // }

#[cfg(test)]
mod test {
    use super::*;
    use reqwest::{header::CONTENT_TYPE, Client, StatusCode};
    use std::time::Duration;
    use tokio::{spawn, time::delay_for};

    async fn test() {
        delay_for(Duration::from_millis(100)).await;
        let client = Client::new();

        let res = client.post("http://localhost:8080").send().await.unwrap();

        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(res.text().await.unwrap(), "hello");

        let res = client
            .post("http://localhost:8080")
            .header(CONTENT_TYPE, "")
            .send()
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
        assert_eq!(res.text().await.unwrap(), "");

        let res = client
            .post("http://localhost:8080")
            .header(CONTENT_TYPE, "application/json")
            .send()
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(res.text().await.unwrap(), "hello");

        let res = client
            .post("http://localhost:8080")
            .header(CONTENT_TYPE, "application/hal+json")
            .send()
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(res.text().await.unwrap(), "hello");

        let res = client
            .post("http://localhost:8080")
            .header(CONTENT_TYPE, "a")
            .send()
            .await
            .unwrap();

        assert_eq!(res.status(), StatusCode::UNSUPPORTED_MEDIA_TYPE);
        assert_eq!(res.text().await.unwrap(), "");
    }

    #[cfg(feature = "router-gotham")]
    #[tokio::test]
    async fn test_gotham() {
        use gotham::{
            init_server,
            pipeline::{new_pipeline, single::single_pipeline},
            router::builder::{build_router, DefineSingleRoute, DrawRoutes},
            state::State,
        };

        spawn(async {
            fn hello(state: State) -> (State, &'static str) {
                (state, "hello")
            }

            let (chain, pipelines) = single_pipeline(
                new_pipeline()
                    .add(ContentType::default().multipart())
                    .build(),
            );

            let router = build_router(chain, pipelines, |route| {
                route.post("/").to(hello);
            });

            init_server("127.0.0.1:8080", router).await.unwrap()
        });

        test().await
    }
}
