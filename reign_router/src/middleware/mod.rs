//! Contains some common middlewares

use crate::{HandleFuture, MiddlewareItem, Request};
use std::sync::Arc;

/// Represents a type which can be used as a middleware
///
/// ```
/// use reign::router::{Chain, HandleFuture, Middleware, Request, futures::FutureExt};
///
/// pub struct Logger {}
///
/// impl Middleware for Logger {
///     fn handle<'m>(&'m self, req: &'m mut Request, chain: Chain<'m>) -> HandleFuture<'m> {
///         async move {
///             println!("Request: from {} on {}", req.ip(), req.uri().path());
///             let response = chain.run(req).await?;
///             println!("Response: status {}", response.status());
///
///             Ok(response)
///         }.boxed()
///     }
/// }
/// ```
pub trait Middleware {
    // TODO: external:rust: Async trait
    // TODO: ergonomics: Look at conduit's middleware for inspiration
    /// Handler for the main logic in the middleware
    fn handle<'m>(&'m self, req: &'m mut Request, chain: Chain<'m>) -> HandleFuture<'m>;
}

/// Middleware chain passed to a middleware handler
pub struct Chain<'a> {
    #[allow(clippy::borrowed_box)]
    pub(crate) handler: &'a Box<dyn Fn(&mut Request) -> HandleFuture + Send + Sync + 'static>,
    pub(crate) middlewares: &'a [Arc<MiddlewareItem>],
}

impl<'a> Chain<'a> {
    /// Asynchronously execute the remaining middleware chain.
    pub fn run(mut self, req: &'a mut Request) -> HandleFuture<'a> {
        if let Some((current, chain)) = self.middlewares.split_first() {
            self.middlewares = chain;
            current.handle(req, self)
        } else {
            (self.handler)(req)
        }
    }
}

mod content_type;
mod headers_default;
mod request_logger;
mod runtime;

#[cfg(feature = "cookie")]
pub mod cookie;
#[cfg(feature = "session")]
pub mod session;

pub use content_type::ContentType;
pub use headers_default::HeadersDefault;
pub use request_logger::RequestLogger;
pub use runtime::Runtime;
