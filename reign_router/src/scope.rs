use crate::{Constraint, Path, Request, RouteRef, Router};
use std::sync::Arc;

/// Scope can be used to define common path prefix, middleware or constraints for routes
///
/// # Examples
///
/// ```
/// use reign::router::{Router, Scope};
/// # use reign::prelude::*;
/// #
/// # #[action]
/// # async fn foo(req: &mut Request) -> Result<impl Response, Error> { Ok("foo") }
///
/// fn router(r: &mut Router) {
///     r.scope_as(Scope::new("api").to(|r| {
///         r.get("foo", foo);
///     }));
/// }
/// ```
#[derive(Default)]
pub struct Scope<'a> {
    pub(crate) path: Path<'a>,
    pub(crate) pipes: Vec<&'a str>,
    pub(crate) router: Router<'a>,
    pub(crate) constraint: Option<Arc<Constraint>>,
}

impl<'a> Scope<'a> {
    /// Define an empty path prefix for this scope
    ///
    /// This is used when you want to group some routes under a middleware pipe or
    /// some constraint but don't want to alter their paths.
    ///
    /// # Examples
    ///
    /// ```
    /// use reign::router::{Router, Scope};
    /// # use reign::prelude::*;
    /// #
    /// # #[action]
    /// # async fn foo(req: &mut Request) -> Result<impl Response, Error> { Ok("foo") }
    ///
    /// fn router(r: &mut Router) {
    ///     r.scope_as(Scope::empty().to(|r| {
    ///         r.get("foo", foo);
    ///     }));
    /// }
    /// ```
    pub fn empty() -> Self {
        Self::default()
    }

    /// Define the path prefix for this scope
    ///
    /// # Examples
    ///
    /// ```
    /// use reign::router::{Router, Scope};
    /// # use reign::prelude::*;
    /// #
    /// # #[action]
    /// # async fn foo(req: &mut Request) -> Result<impl Response, Error> { Ok("foo") }
    ///
    /// fn router(r: &mut Router) {
    ///     r.scope_as(Scope::new("api").to(|r| {
    ///         r.get("foo", foo);
    ///     }));
    /// }
    /// ```
    pub fn new<P>(path: P) -> Self
    where
        P: Into<Path<'a>>,
    {
        Self {
            path: path.into(),
            ..Default::default()
        }
    }

    /// Define the middlewares that run for all the routes under this scope
    ///
    /// # Examples
    ///
    /// ```
    /// use reign::router::{Router, Scope, middleware::Runtime};
    /// # use reign::prelude::*;
    /// #
    /// # #[action]
    /// # async fn foo(req: &mut Request) -> Result<impl Response, Error> { Ok("foo") }
    ///
    /// fn router(r: &mut Router) {
    ///     r.pipe("common").add(Runtime::default());
    ///
    ///     r.scope_as(Scope::new("api").through(&["common"]).to(|r| {
    ///         r.get("foo", foo);
    ///     }));
    /// }
    /// ```
    pub fn through(mut self, pipes: &[&'a str]) -> Self {
        self.pipes = pipes.to_vec();
        self
    }

    /// Define the routes that exist under this scope
    ///
    /// Even though you can define a scope without this, it will do nothing and won't affect routing.
    ///
    /// # Examples
    ///
    /// ```
    /// use reign::router::{Router, Scope};
    /// # use reign::prelude::*;
    /// #
    /// # #[action]
    /// # async fn foo(req: &mut Request) -> Result<impl Response, Error> { Ok("foo") }
    ///
    /// fn router(r: &mut Router) {
    ///     r.scope_as(Scope::new("api").to(|r| {
    ///         r.get("foo", foo);
    ///     }));
    /// }
    /// ```
    pub fn to<R>(mut self, f: R) -> Self
    where
        R: Fn(&mut Router),
    {
        let mut router = Router::default();
        f(&mut router);

        self.router = router;
        self
    }

    /// Define the constraint that restricts matching for routes under this scope
    ///
    /// # Examples
    ///
    /// ```
    /// use reign::router::{Router, Scope};
    /// # use reign::prelude::*;
    /// #
    /// # #[action]
    /// # async fn foo(req: &mut Request) -> Result<impl Response, Error> { Ok("foo") }
    ///
    /// fn router(r: &mut Router) {
    ///     r.scope_as(Scope::new("api").constraint(|req| {
    ///         req.query("token").is_some()
    ///     }).to(|r| {
    ///         r.get("foo", foo);
    ///     }));
    /// }
    /// ```
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
