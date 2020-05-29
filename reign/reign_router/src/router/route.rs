use crate::router::{
    hyper::{Body, Method, Response},
    Path, Request,
};
use anyhow::Error;
use futures::prelude::*;

type Handler = Box<dyn Fn(Request) -> HandlerReturn + Send + Sync + 'static>;
type HandlerReturn = Box<dyn Future<Output = Result<Response<Body>, Error>> + Send + 'static>;

#[derive(Default)]
pub struct Route<'a> {
    pub(crate) path: Path<'a>,
    pub(crate) methods: Vec<Method>,
    pub(crate) handler: Option<Handler>,
}

impl<'a> Route<'a> {
    pub fn new(path: Path<'a>) -> Self {
        let mut ret = Self::default();
        ret.path = path;
        ret
    }

    pub fn methods(mut self, methods: &[Method]) -> Self {
        self.methods = methods.to_vec();
        self
    }

    pub fn handler<H, R>(mut self, handler: H) -> Self
    where
        H: Fn(Request) -> R + Send + Sync + 'static,
        R: Future<Output = Result<Response<Body>, Error>> + Send + 'static,
    {
        let handler: Handler = Box::new(move |req: Request| Box::new(handler(req)));
        self.handler = Some(Box::new(handler));
        self
    }
}
