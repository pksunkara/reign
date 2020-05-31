#[cfg(feature = "router-actix")]
use actix_service::{Service, Transform};
#[cfg(feature = "router-actix")]
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    Error,
};
use chrono::prelude::Utc;
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

fn dur_to_string(i: i64) -> String {
    if i < 1000 {
        format!("{}us", i)
    } else if i < 1_000_000 {
        format!("{:.2}ms", (i as f32) / 1000.0)
    } else {
        format!("{:.2}s", (i as f32) / 1_000_000.0)
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "router-gotham", derive(NewMiddleware))]
pub struct Runtime {
    header: &'static str,
}

impl crate::router::Middleware for Runtime {
    fn handle<'m>(
        &'m self,
        req: &'m mut crate::router::Request,
        chain: crate::router::Chain<'m>,
    ) -> Pin<
        Box<
            dyn Future<
                    Output = Result<
                        crate::router::hyper::Response<crate::router::hyper::Body>,
                        crate::router::Error,
                    >,
                > + Send
                + 'm,
        >,
    > {
        async move {
            let start = Utc::now();
            let mut response = chain.run(req).await?;
            let duration = Utc::now().signed_duration_since(start).num_microseconds();

            if let Some(dur) = duration {
                response.headers_mut().insert(
                    self.header,
                    crate::router::hyper::header::HeaderValue::from_str(&dur_to_string(dur))
                        .unwrap(),
                );
            }

            Ok(response)
        }
        .boxed()
    }
}

impl Runtime {
    pub fn new(header: &'static str) -> Self {
        if header.to_lowercase() != header {
            panic!("Only lowercase headers are allowed for Runtime middleware");
        }

        Self { header }
    }

    pub fn default() -> Self {
        Self::new("x-runtime")
    }
}

#[cfg(feature = "router-actix")]
pub struct RuntimeMiddleware<S> {
    service: S,
    inner: Runtime,
}

#[cfg(feature = "router-actix")]
impl<S, B> Transform<S> for Runtime
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RuntimeMiddleware<S>;
    type Future = future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        future::ok(RuntimeMiddleware {
            service,
            inner: self.clone(),
        })
    }
}

#[cfg(feature = "router-actix")]
#[allow(clippy::type_complexity)]
impl<S, B> Service for RuntimeMiddleware<S>
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

        let start = Utc::now();
        let fut = self.service.call(req);
        let header = self.inner.header.as_bytes();

        async move {
            let mut res = fut.await?;
            let duration = Utc::now().signed_duration_since(start).num_microseconds();

            if let Some(dur) = duration {
                res.headers_mut().insert(
                    HeaderName::from_lowercase(header).map_err(HttpError::from)?,
                    HeaderValue::from_str(&dur_to_string(dur)).map_err(HttpError::from)?,
                );
            }

            Ok(res)
        }
        .boxed_local()
    }
}

#[cfg(feature = "router-gotham")]
impl gotham::middleware::Middleware for Runtime {
    fn call<Chain>(self, state: State, chain: Chain) -> Pin<Box<HandlerFuture>>
    where
        Chain: FnOnce(State) -> Pin<Box<HandlerFuture>> + Send + 'static,
    {
        use gotham::hyper::header::HeaderValue;

        let start = Utc::now();

        chain(state)
            .and_then(move |(state, mut response)| {
                let duration = Utc::now().signed_duration_since(start).num_microseconds();

                if let Some(dur) = duration {
                    response.headers_mut().insert(
                        self.header,
                        HeaderValue::from_str(&dur_to_string(dur)).unwrap(),
                    );
                }

                future::ok((state, response))
            })
            .boxed()
    }
}

#[cfg(feature = "router-tide")]
impl<S> tide::Middleware<S> for Runtime
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
            let start = Utc::now();
            let mut response = next.run(ctx).await?;
            let duration = Utc::now().signed_duration_since(start).num_microseconds();

            if let Some(dur) = duration {
                response = response.set_header(
                    HeaderName::from_ascii(self.header.as_bytes().to_vec())?,
                    dur_to_string(dur),
                );
            }

            Ok(response)
        }
        .boxed()
    }
}

#[cfg(test)]
mod test {
    use super::{dur_to_string, Runtime};

    #[test]
    fn test_dur_to_string_micro_seconds() {
        assert_eq!(&dur_to_string(193), "193us");
    }

    #[test]
    fn test_dur_to_string_milli_seconds() {
        assert_eq!(&dur_to_string(2940), "2.94ms");
    }

    #[test]
    fn test_dur_to_string_seconds() {
        assert_eq!(&dur_to_string(3495773), "3.50s");
    }

    #[test]
    #[should_panic]
    fn test_runtime_with_uppercase() {
        Runtime::new("X-Runtime");
    }
}
