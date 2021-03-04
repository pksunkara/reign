use crate::{
    futures::FutureExt,
    hyper::{Body, Response as HyperResponse},
    Error, Request, Response,
};

use std::{future::Future, pin::Pin};

/// Return type of a middleware handle or an endpoint handle when `action` attribute is not used
pub type HandleFuture<'a> =
    Pin<Box<dyn Future<Output = Result<HyperResponse<Body>, Error>> + Send + 'a>>;

pub trait AsyncFn<'a>: Send + Sync + 'static {
    fn call(&'a self, req: &'a mut Request) -> HandleFuture<'a>;
}

impl<'a, T, F, R, E> AsyncFn<'a> for T
where
    T: Fn(&'a mut Request) -> F + Send + Sync + 'static,
    F: Future<Output = Result<R, E>> + Send + 'a,
    R: Response,
    E: Response,
{
    fn call(&'a self, req: &'a mut Request) -> HandleFuture<'a> {
        async move {
            let result = (self)(req).await;

            match result {
                Ok(r) => Ok(r.respond()?),
                Err(e) => Ok(e.respond()?),
            }
        }
        .boxed()
    }
}

pub trait Handle: Send + Sync + 'static {
    fn call<'a>(&'a self, req: &'a mut Request) -> HandleFuture<'a>;
}

impl<T> Handle for T
where
    T: for<'r> AsyncFn<'r>,
{
    fn call<'a>(&'a self, req: &'a mut Request) -> HandleFuture<'a> {
        self.call(req)
    }
}
