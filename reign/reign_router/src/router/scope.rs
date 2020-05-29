use crate::router::{Path, Router};

#[derive(Default)]
pub struct Scope<'a> {
    pub(crate) path: Path<'a>,
    pub(crate) pipes: Vec<&'a str>,
    pub(crate) router: Router<'a>,
}

impl<'a> Scope<'a> {
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn new(path: Path<'a>) -> Self {
        let mut ret = Self::default();
        ret.path = path;
        ret
    }

    pub fn through(mut self, pipes: &[&'a str]) -> Self {
        self.pipes = pipes.to_vec();
        self
    }

    pub fn to<R>(mut self, router_fn: R) -> Self
    where
        R: Fn(&mut Router),
    {
        let mut router = Router::default();
        router_fn(&mut router);

        self.router = router;
        self
    }
}
