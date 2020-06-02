use crate::Middleware;
use std::sync::Arc;

pub(crate) type MiddlewareItem = Box<dyn Middleware + Send + Sync + 'static>;

pub struct Pipe<'a> {
    pub(crate) name: &'a str,
    pub(crate) middlewares: Vec<Arc<MiddlewareItem>>,
}

impl<'a> Pipe<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            middlewares: vec![],
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn add<M>(mut self, middleware: M) -> Self
    where
        M: Middleware + Send + Sync + 'static,
    {
        self.middlewares.push(Arc::new(Box::new(middleware)));
        self
    }
}
