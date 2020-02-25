#[cfg(feature = "router-tide")]
use futures::future::BoxFuture;
#[cfg(feature = "router-gotham")]
use gotham::{
    handler::HandlerFuture,
    helpers::http::response::create_empty_response,
    hyper::{header, HeaderMap, StatusCode},
    state::{FromState, State},
};
use mime::{Mime, Name, FORM_DATA, JSON, WWW_FORM_URLENCODED};
#[cfg(feature = "router-tide")]
use tide::{middleware::Next, Request, Response};

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

#[cfg(feature = "router-tide")]
impl<'a, S> tide::middleware::Middleware<S> for ContentType<'a>
where
    S: Send + Sync + 'a,
    'a: 'static,
{
    fn handle<'b>(&'b self, ctx: Request<S>, next: Next<'b, S>) -> BoxFuture<'b, Response> {
        Box::pin(async move { next.run(ctx).await })
    }
}
