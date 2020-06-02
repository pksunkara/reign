use crate::{HandleFuture, MiddlewareItem, Request};
use std::sync::Arc;

pub trait Middleware {
    fn handle<'m>(&'m self, req: &'m mut Request, chain: Chain<'m>) -> HandleFuture<'m>;
}

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

pub mod cookie_parser;
pub mod session;

pub use content_type::ContentType;
pub use headers_default::HeadersDefault;
pub use request_logger::RequestLogger;
pub use runtime::Runtime;
