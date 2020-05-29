use crate::router::Middleware;

pub struct Pipe<'a> {
    pub(crate) name: &'a str,
    pub(crate) middlewares: Vec<Box<dyn Middleware + 'a>>,
}

impl<'a> Pipe<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            middlewares: vec![],
        }
    }

    pub fn and<M>(mut self, middleware: M) -> Self
    where
        M: Middleware + 'a,
    {
        self.middlewares.push(Box::new(middleware));
        self
    }
}
