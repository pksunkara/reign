#[cfg(feature = "router-actix")]
use actix_service::{Service, Transform};
#[cfg(feature = "router-actix")]
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    Error, HttpResponse,
};
use futures::prelude::*;
#[cfg(feature = "router-gotham")]
use gotham::{
    handler::HandlerFuture,
    helpers::http::response::create_empty_response,
    state::{FromState, State},
};
#[cfg(feature = "router-gotham")]
use gotham_derive::NewMiddleware;
use mime::{Mime, Name, FORM_DATA, JSON, WWW_FORM_URLENCODED};
use std::pin::Pin;
#[cfg(feature = "router-actix")]
use std::task::{Context, Poll};
#[cfg(feature = "router-tide")]
use tide::{Next, Request, Response};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "router-gotham", derive(NewMiddleware))]
pub struct ContentType<'a> {
    subtypes: Vec<&'a str>, // TODO:(lifetime) Do I need the lifetime here?
}

impl<'a> ContentType<'a> {
    pub fn new(subtypes: Vec<&'a str>) -> Self {
        Self { subtypes }
    }

    pub fn default() -> Self {
        Self::empty().json().form()
    }

    pub fn empty() -> Self {
        Self::new(vec![])
    }

    pub fn json(mut self) -> Self {
        self.subtypes.push(JSON.as_str());
        self
    }

    pub fn form(mut self) -> Self {
        self.subtypes.push(WWW_FORM_URLENCODED.as_str());
        self
    }

    pub fn multipart(mut self) -> Self {
        self.subtypes.push(FORM_DATA.as_str());
        self
    }

    fn allow(&self, val: Name) -> bool {
        self.subtypes.iter().any(|&x| x == val.as_str())
    }
}

#[cfg(feature = "router-actix")]
pub struct ContentTypeMiddleware<'a, S> {
    service: S,
    inner: ContentType<'a>,
}

#[cfg(feature = "router-actix")]
impl<'a, S, B> Transform<S> for ContentType<'a>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = ContentTypeMiddleware<'a, S>;
    type Future = future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        future::ok(ContentTypeMiddleware {
            service,
            inner: self.clone(),
        })
    }
}

#[cfg(feature = "router-actix")]
#[allow(clippy::type_complexity)]
impl<'a, S, B> Service for ContentTypeMiddleware<'a, S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + 'static>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        use actix_web::http::header::CONTENT_TYPE;

        match req.headers().get(CONTENT_TYPE) {
            Some(content_type) => {
                if let Ok(content_type) = content_type.to_str() {
                    if let Ok(val) = content_type.parse::<Mime>() {
                        if self.inner.allow(val.subtype()) {
                            return self.service.call(req).boxed_local();
                        }

                        if let Some(suffix) = val.suffix() {
                            if self.inner.allow(suffix) {
                                return self.service.call(req).boxed_local();
                            }
                        }
                    }
                }
            }
            None => {
                return self.service.call(req).boxed_local();
            }
        }

        let response = req.into_response(
            HttpResponse::UnsupportedMediaType()
                .finish()
                .into_body::<B>(),
        );
        async { Ok(response) }.boxed_local()
    }
}

#[cfg(feature = "router-gotham")]
impl<'a> gotham::middleware::Middleware for ContentType<'a> {
    fn call<Chain>(self, state: State, chain: Chain) -> Pin<Box<HandlerFuture>>
    where
        Chain: FnOnce(State) -> Pin<Box<HandlerFuture>> + Send + 'static,
    {
        use gotham::hyper::{header::CONTENT_TYPE, HeaderMap, StatusCode};

        match HeaderMap::borrow_from(&state).get(CONTENT_TYPE) {
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

#[cfg(feature = "router-tide")]
impl<'a, S> tide::Middleware<S> for ContentType<'a>
where
    S: Send + Sync + 'a,
    'a: 'static,
{
    fn handle<'b>(
        &'b self,
        ctx: Request<S>,
        next: Next<'b, S>,
    ) -> Pin<Box<dyn Future<Output = tide::Result<Response>> + Send + 'b>> {
        use tide::http::{headers::CONTENT_TYPE, StatusCode};

        match ctx.header(&CONTENT_TYPE) {
            Some(content_type) => {
                if content_type.len() == 1 {
                    if let Ok(val) = content_type.get(0).unwrap().as_str().parse::<Mime>() {
                        if self.allow(val.subtype()) {
                            return next.run(ctx);
                        }

                        if let Some(suffix) = val.suffix() {
                            if self.allow(suffix) {
                                return next.run(ctx);
                            }
                        }
                    }
                }
            }
            None => {
                return next.run(ctx);
            }
        };

        let response = Response::new(StatusCode::UnsupportedMediaType);
        async { Ok(response) }.boxed()
    }
}
