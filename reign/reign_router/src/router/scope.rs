use crate::router::{Constraint, Index, Path, Request, Router};

#[derive(Default)]
pub struct Scope<'a> {
    pub(crate) path: Path<'a>,
    pub(crate) pipes: Vec<&'a str>,
    pub(crate) router: Router<'a>,
    pub(crate) constraint: Option<Constraint>,
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

    pub fn to<R>(mut self, router_fn: R) -> Self
    where
        R: Fn(&mut Router),
    {
        let mut router = Router::in_scope();
        router_fn(&mut router);

        self.router = router;
        self
    }

    pub fn constraint<C>(mut self, constraint: C) -> Self
    where
        C: Fn(Request) -> bool + Send + Sync + 'static,
    {
        self.constraint = Some(Box::new(constraint));
        self
    }

    pub(crate) fn regex(&self) -> (String, Vec<(Vec<Index>, (String, String))>) {
        (self.path.regex(), self.router.regex())
    }
}
