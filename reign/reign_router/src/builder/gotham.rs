#![allow(clippy::new_without_default)]

use super::RouterTypeTrait;
use gotham::{
    middleware::{chain::NewMiddlewareChain, Middleware, NewMiddleware},
    pipeline::{
        new_pipeline,
        set::{new_pipeline_set, EditablePipelineSet},
        Pipeline, PipelineBuilder,
    },
};

pub struct RouterTypeGotham;

impl RouterTypeTrait for RouterTypeGotham {
    const TYPE: &'static str = "gotham";
}

enum PipelineOrBuilder<U>
where
    U: NewMiddlewareChain,
{
    Pipeline(Pipeline<U>),
    Builder(PipelineBuilder<U>),
}

pub struct Pipe<RouterTypeGotham, U>
where
    U: NewMiddlewareChain,
{
    name: &'static str,
    inner: PipelineOrBuilder<U>,
    _phantom: RouterTypeGotham,
}

impl Pipe<RouterTypeGotham, ()> {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            inner: PipelineOrBuilder::Builder(new_pipeline()),
            _phantom: RouterTypeGotham,
        }
    }
}

impl<C> Pipe<RouterTypeGotham, C>
where
    C: NewMiddlewareChain,
{
    pub fn and<T>(self, middleware: T) -> Pipe<RouterTypeGotham, (T, C)>
    where
        T: NewMiddleware,
        T::Instance: Middleware + Send + 'static,
    {
        if let PipelineOrBuilder::Builder(x) = self.inner {
            Pipe::<RouterTypeGotham, (T, C)> {
                name: self.name,
                inner: PipelineOrBuilder::Builder(x.add(middleware)),
                _phantom: self._phantom,
            }
        } else {
            panic!("no builder found");
        }
    }

    pub fn build(self) -> Self {
        if let PipelineOrBuilder::Builder(x) = self.inner {
            Self {
                name: self.name,
                inner: PipelineOrBuilder::Pipeline(x.build()),
                _phantom: self._phantom,
            }
        } else {
            panic!("no builder found");
        }
    }
}

pub struct Pipes<RouterTypeGotham, U> {
    inner: EditablePipelineSet<U>,
    _phantom: RouterTypeGotham,
}

impl Pipes<RouterTypeGotham, ()> {
    pub fn new() -> Self {
        Self {
            inner: new_pipeline_set(),
            _phantom: RouterTypeGotham,
        }
    }
}

impl<U> Pipes<RouterTypeGotham, U> {
    pub fn pipe<T>(self, pipe: Pipe<RouterTypeGotham, T>) -> Pipes<RouterTypeGotham, U::Output>
    where
        T: NewMiddlewareChain,
        U: borrow_bag::Append<Pipe<RouterTypeGotham, T>> + std::any::Any,
    {
        let (inner, _handle) = self.inner.add(pipe);

        println!("{:#?}", std::any::TypeId::of::<U>());

        Pipes::<RouterTypeGotham, U::Output> {
            inner,
            _phantom: self._phantom,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::middleware::{ContentType, HeadersDefault, Runtime};

    #[test]
    fn test_builder() {
        let a = Pipe::<RouterTypeGotham, _>::new("common")
            .and(HeadersDefault::empty().add("x-1", "a"))
            .and(ContentType::empty().json())
            .build();

        let b = Pipe::<RouterTypeGotham, _>::new("timer")
            .and(Runtime::default())
            .build();

        let _pipes = Pipes::<RouterTypeGotham, _>::new().pipe(a).pipe(b);
    }
}
