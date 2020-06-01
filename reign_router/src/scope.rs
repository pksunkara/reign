use crate::{Constraint, Path, Request, RouteRef, Router};
use std::sync::Arc;

#[derive(Default)]
pub struct Scope<'a> {
    pub(crate) path: Path<'a>,
    pub(crate) pipes: Vec<&'a str>,
    pub(crate) router: Router<'a>,
    pub(crate) constraint: Option<Arc<Constraint>>,
}

impl<'a> Scope<'a> {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn new<P>(path: P) -> Self
    where
        P: Into<Path<'a>>,
    {
        Self {
            path: path.into(),
            ..Default::default()
        }
    }

    pub fn through(mut self, pipes: &[&'a str]) -> Self {
        self.pipes = pipes.to_vec();
        self
    }

    pub fn to<R>(mut self, f: R) -> Self
    where
        R: Fn(&mut Router),
    {
        let mut router = Router::in_scope();
        f(&mut router);

        self.router = router;
        self
    }

    pub fn constraint<C>(mut self, constraint: C) -> Self
    where
        C: Fn(&Request) -> bool + Send + Sync + 'static,
    {
        self.constraint = Some(Arc::new(Box::new(constraint)));
        self
    }

    pub(crate) fn regex(&self) -> (String, Vec<(String, String)>) {
        (self.path.regex(), self.router.regex())
    }

    pub(crate) fn refs(&self) -> (Option<Arc<Constraint>>, Vec<RouteRef>, Vec<&str>) {
        (
            self.constraint.clone(),
            self.router.refs(),
            self.pipes.clone(),
        )
    }
}
