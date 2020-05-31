use crate::{
    hyper::{Body, Method, Response},
    Error, Path, Request,
};
use std::{future::Future, pin::Pin, sync::Arc};

pub(crate) type Handler = Box<dyn Fn(&mut Request) -> HandleFuture + Send + Sync + 'static>;
pub type HandleFuture<'a> =
    Pin<Box<dyn Future<Output = Result<Response<Body>, Error>> + Send + 'a>>;
pub(crate) type Constraint = Box<dyn Fn(&Request) -> bool + Send + Sync + 'static>;

#[derive(Default, Clone)]
pub(crate) struct Route<'a> {
    pub(crate) path: Path<'a>,
    pub(crate) methods: Vec<Method>,
    pub(crate) handler: Option<Arc<Handler>>,
    pub(crate) constraint: Option<Arc<Constraint>>,
}

impl<'a> Route<'a> {
    pub(crate) fn new<P>(path: P) -> Self
    where
        P: Into<Path<'a>>,
    {
        Self {
            path: path.into(),
            ..Default::default()
        }
    }

    pub(crate) fn methods(mut self, methods: &[Method]) -> Self {
        self.methods = methods.to_vec();
        self
    }

    pub(crate) fn handler<H>(mut self, handler: H) -> Self
    where
        H: Fn(&mut Request) -> HandleFuture + Send + Sync + 'static,
    {
        self.handler = Some(Arc::new(Box::new(handler)));
        self
    }

    pub(crate) fn constraint<C>(mut self, constraint: C) -> Self
    where
        C: Fn(&Request) -> bool + Send + Sync + 'static,
    {
        self.constraint = Some(Arc::new(Box::new(constraint)));
        self
    }

    pub(crate) fn regex(&self) -> (String, String) {
        let methods = if self.methods.is_empty() {
            format!("^(?:GET|POST|PUT|PATCH|DELETE|HEAD|OPTIONS|TRACE|CONNECT)")
        } else {
            format!(
                "^(?:{})",
                self.methods
                    .iter()
                    .map(|x| x.as_str())
                    .collect::<Vec<_>>()
                    .join("|")
            )
        };

        (methods, format!("{}$", self.path.regex()))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_regex_single_method() {
        let r = Route::new("").methods(&[Method::GET]);
        assert_eq!(r.regex().0, "^(?:GET)");
    }

    #[test]
    fn test_regex_multi_methods() {
        let r = Route::new("").methods(&[Method::GET, Method::POST]);
        assert_eq!(r.regex().0, "^(?:GET|POST)");
    }

    #[test]
    fn test_regex_all_methods() {
        let r = Route::new("");
        assert_eq!(
            r.regex().0,
            "^(?:GET|POST|PUT|PATCH|DELETE|HEAD|OPTIONS|TRACE|CONNECT)"
        );
    }

    #[test]
    fn test_regex_path() {
        let r = Route::new("");
        assert_eq!(r.regex().1, "$");
    }
}
