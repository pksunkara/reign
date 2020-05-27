#[cfg(feature = "router-actix")]
use actix_service::{Service, Transform};
#[cfg(feature = "router-actix")]
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    Error,
};
use futures::prelude::*;
#[cfg(feature = "router-gotham")]
use gotham::{handler::HandlerFuture, state::State};
#[cfg(feature = "router-gotham")]
use gotham_derive::NewMiddleware;
use std::pin::Pin;
#[cfg(feature = "router-actix")]
use std::task::{Context, Poll};
#[cfg(feature = "router-tide")]
use tide::{Next, Request, Response};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "router-gotham", derive(NewMiddleware))]
pub struct HeadersDefault {
    headers: Vec<(&'static str, &'static str)>,
}

impl HeadersDefault {
    pub fn new(headers: Vec<(&'static str, &'static str)>) -> Self {
        Self { headers }
    }

    pub fn default() -> Self {
        Self::empty().add("x-powered-by", "reign")
    }

    pub fn empty() -> Self {
        Self::new(vec![])
    }

    pub fn add(mut self, name: &'static str, value: &'static str) -> Self {
        if name.to_lowercase() != name {
            panic!("Only lowercase headers are allowed");
        }

        self.headers.push((name, value));
        self
    }
}

#[cfg(feature = "router-actix")]
pub struct HeadersDefaultMiddleware<S> {
    service: S,
    inner: HeadersDefault,
}

#[cfg(feature = "router-actix")]
impl<S, B> Transform<S> for HeadersDefault
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = HeadersDefaultMiddleware<S>;
    type Future = future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        future::ok(HeadersDefaultMiddleware {
            service,
            inner: self.clone(),
        })
    }
}

#[cfg(feature = "router-actix")]
impl<S, B> Service for HeadersDefaultMiddleware<S>
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
        use actix_web::http::{Error as HttpError, HeaderName, HeaderValue};

        let fut = self.service.call(req);
        let headers: Vec<(&[u8], &str)> = self
            .inner
            .headers
            .iter()
            .map(|i| (i.0.as_bytes(), i.1))
            .collect();

        async move {
            let mut res = fut.await?;

            for (name, value) in &headers {
                res.headers_mut().insert(
                    HeaderName::from_lowercase(name).map_err(HttpError::from)?,
                    HeaderValue::from_str(value).map_err(HttpError::from)?,
                )
            }

            Ok(res)
        }
        .boxed_local()
    }
}

#[cfg(feature = "router-gotham")]
impl gotham::middleware::Middleware for HeadersDefault {
    fn call<Chain>(self, state: State, chain: Chain) -> Pin<Box<HandlerFuture>>
    where
        Chain: FnOnce(State) -> Pin<Box<HandlerFuture>> + Send + 'static,
    {
        use gotham::hyper::header::HeaderValue;

        chain(state)
            .and_then(move |(state, mut response)| {
                for (name, value) in &self.headers {
                    response
                        .headers_mut()
                        .insert(*name, HeaderValue::from_str(value).unwrap());
                }

                future::ok((state, response))
            })
            .boxed()
    }
}

#[cfg(feature = "router-tide")]
impl<S> tide::Middleware<S> for HeadersDefault
where
    S: Send + Sync + 'static,
{
    fn handle<'b>(
        &'b self,
        ctx: Request<S>,
        next: Next<'b, S>,
    ) -> Pin<Box<dyn Future<Output = tide::Result<Response>> + Send + 'b>> {
        use tide::http::headers::HeaderName;

        async move {
            let mut response = next.run(ctx).await?;

            for (name, value) in &self.headers {
                response =
                    response.set_header(HeaderName::from_ascii(name.as_bytes().to_vec())?, value);
            }

            Ok(response)
        }
        .boxed()
    }
}

#[cfg(test)]
mod test {
    use super::HeadersDefault;

    #[test]
    #[should_panic]
    fn test_with_uppercase() {
        HeadersDefault::empty().add("X-Version", "0.1");
    }
}
