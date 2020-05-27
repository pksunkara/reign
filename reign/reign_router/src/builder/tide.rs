use super::RouterTypeTrait;
use std::{
    collections::HashMap as Map,
    fmt::{Debug, Formatter, Result as FmtResult},
    future::Future,
    marker::PhantomData,
    pin::Pin,
};
use tide::{Middleware, Next, Request, Result, Server};

pub struct RouterTypeTide;

impl RouterTypeTrait for RouterTypeTide {
    const TYPE: &'static str = "tide";
}

pub struct Wrapper<T> {
    inner: T,
}

trait DebuggableMiddleware<S> {
    fn handle<'a>(
        &'a self,
        cx: Request<S>,
        next: Next<'a, S>,
    ) -> Pin<Box<dyn Future<Output = Result> + Send + 'a>>;

    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult;
}

impl<T, S> DebuggableMiddleware<S> for Wrapper<T>
where
    T: Middleware<S> + Debug,
    S: Send + Sync + 'static,
{
    fn handle<'a>(
        &'a self,
        cx: Request<S>,
        next: Next<'a, S>,
    ) -> Pin<Box<dyn Future<Output = Result> + Send + 'a>> {
        self.inner.handle(cx, next)
    }

    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        self.inner.fmt(f)
    }
}

// impl<S> Middleware<S> for Box<dyn DebuggableMiddleware<S> + Send + Sync>
// where
//     S: Send + Sync + 'static,
// {
//     fn handle<'a>(
//         &'a self,
//         cx: Request<S>,
//         next: Next<'a, S>,
//     ) -> Pin<Box<dyn Future<Output = Result> + Send + 'a>> {
//         self.handle(cx, next)
//     }
// }

// impl<S> Debug for Box<dyn DebuggableMiddleware<S> + Send + Sync>
// where
//     S: Send + Sync + 'static,
// {
//     fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
//         self.fmt(f)
//     }
// }

pub struct Pipe<RouterTypeTide, S> {
    name: &'static str,
    inner: Vec<Box<dyn DebuggableMiddleware<S>>>,
    _phantom: PhantomData<RouterTypeTide>,
}

impl<S> Pipe<RouterTypeTide, S>
where
    S: Send + Sync + 'static,
{
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            inner: vec![],
            _phantom: PhantomData,
        }
    }

    pub fn and<M>(mut self, middleware: M) -> Self
    where
        M: Middleware<S> + Debug,
        S: 'static,
    {
        self.inner.push(Box::new(Wrapper { inner: middleware }));

        self
    }

    pub fn build(self) -> Self {
        self
    }
}

pub struct Pipes<RouterTypeTide, S> {
    inner: Map<&'static str, Pipe<RouterTypeTide, S>>,
}

impl<S> Pipes<RouterTypeTide, S>
where
    S: Send + Sync + 'static,
{
    pub fn new() -> Self {
        Self { inner: Map::new() }
    }

    pub fn pipe(mut self, pipe: Pipe<RouterTypeTide, S>) -> Self {
        self.inner.insert(pipe.name, pipe);
        self
    }

    pub fn app(&self, pipes: &'static [&'static str], _app: &mut Server<S>) {
        for pipe in pipes {
            if let Some(p) = self.inner.get(pipe) {
                for _m in &p.inner {
                    // _app.middleware(_m);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::middleware::{ContentType, HeadersDefault, Runtime};

    #[test]
    fn test_builder() {
        let pipes = Pipes::<RouterTypeTide, _>::new()
            .pipe(
                Pipe::new("common")
                    .and(HeadersDefault::empty().add("x-1", "a"))
                    .and(ContentType::empty().json())
                    .build(),
            )
            .pipe(Pipe::new("timer").and(Runtime::default()).build());

        let mut app = tide::new();

        pipes.app(&["common"], &mut app);

        app.at("/").get(|_| async move { Ok("hello") });
    }
}
