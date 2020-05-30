use crate::router::{
    hyper::{Body, Method, Response},
    Error, Path, Request,
};
use futures::prelude::*;

pub(crate) type Handler = Box<dyn Fn(Request) -> HandlerReturn + Send + Sync + 'static>;
pub(crate) type HandlerReturn =
    Box<dyn Future<Output = Result<Response<Body>, Error>> + Send + 'static>;
pub(crate) type Constraint = Box<dyn Fn(Request) -> bool + Send + Sync + 'static>;

#[derive(Default)]
pub(crate) struct Route<'a> {
    pub(crate) path: Path<'a>,
    pub(crate) methods: Vec<Method>,
    pub(crate) handler: Option<Handler>,
    pub(crate) constraint: Option<Constraint>,
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

    pub(crate) fn handler<H, R>(mut self, handler: H) -> Self
    where
        H: Fn(Request) -> R + Send + Sync + 'static,
        R: Future<Output = Result<Response<Body>, Error>> + Send + 'static,
    {
        let handler: Handler = Box::new(move |req: Request| Box::new(handler(req)));
        self.handler = Some(Box::new(handler));
        self
    }

    pub(crate) fn constraint<C>(mut self, constraint: C) -> Self
    where
        C: Fn(Request) -> bool + Send + Sync + 'static,
    {
        self.constraint = Some(Box::new(constraint));
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
