use crate::router::{
    hyper::{Body, Response},
    Error, Handler, MiddlewareItem, Request,
};
use futures::future::BoxFuture;
use std::{future::Future, pin::Pin, sync::Arc};

pub trait Middleware {
    fn handle<'a>(
        &'a self,
        req: &'a mut Request,
        chain: Chain<'a>,
    ) -> Pin<Box<dyn Future<Output = Result<Response<Body>, Error>> + Send + 'a>>;
}

impl<F> Middleware for F
where
    F: for<'a> Fn(&mut Request, Chain<'a>) -> BoxFuture<'a, Result<Response<Body>, Error>>
        + Send
        + Sync
        + 'static,
{
    fn handle<'a>(
        &'a self,
        req: &'a mut Request,
        chain: Chain<'a>,
    ) -> BoxFuture<'a, Result<Response<Body>, Error>> {
        (self)(req, chain)
    }
}

pub struct Chain<'a> {
    pub(crate) handler: &'a Handler,
    pub(crate) middlewares: &'a [Arc<MiddlewareItem>],
}

impl<'a> Chain<'a> {
    /// Asynchronously execute the remaining middleware chain.
    pub fn run(mut self, req: &'a mut Request) -> BoxFuture<'a, Result<Response<Body>, Error>> {
        if let Some((current, chain)) = self.middlewares.split_first() {
            self.middlewares = chain;
            current.handle(req, self)
        } else {
            Pin::from((self.handler)(req))
        }
    }
}
