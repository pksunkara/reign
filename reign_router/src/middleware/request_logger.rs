use crate::{middleware::runtime::dur_to_string, Chain, HandleFuture, Middleware, Request};
use chrono::prelude::Utc;
use futures::FutureExt;
use hyper::header::CONTENT_LENGTH;
use log::{log, log_enabled, Level};

/// Logs the request and then response if possible
pub struct RequestLogger {
    level: Level,
}

impl RequestLogger {
    /// Instantiates the middleware with log level
    ///
    /// # Examples
    ///
    /// ```
    /// use reign::{router::{Router, middleware::RequestLogger}, log::Level};
    ///
    /// fn router(r: &mut Router) {
    ///     r.pipe("common").add(RequestLogger::new(Level::Info));
    /// }
    /// ```
    pub fn new(level: Level) -> Self {
        RequestLogger { level }
    }
}

impl Middleware for RequestLogger {
    fn handle<'m>(&'m self, req: &'m mut Request, chain: Chain<'m>) -> HandleFuture<'m> {
        if !log_enabled!(self.level) {
            return chain.run(req);
        }

        async move {
            let start = Utc::now();

            log!(
                target: "reign_router",
                self.level,
                "{} {}",
                req.method(),
                req.uri().path(),
            );

            let response = chain.run(req).await?;

            let length = response
                .headers()
                .get(CONTENT_LENGTH)
                .map(|len| len.to_str().unwrap())
                .unwrap_or("0");

            let duration = Utc::now().signed_duration_since(start).num_microseconds();

            log!(
                target: "reign_router",
                self.level,
                "{} - {} - {}",
                response.status(),
                length,
                dur_to_string(duration.unwrap_or(0)),
            );

            Ok(response)
        }
        .boxed()
    }
}
