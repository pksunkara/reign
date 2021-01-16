use crate::{Constraint, Path, Pipe, Request, RouteRef, Router};

use std::{collections::HashMap as Map, sync::Arc};

/// Scope can be used to define common path prefixes, middlewares or constraints for routes
///
/// # Examples
///
/// ```
/// use reign::router::Router;
/// # use reign::prelude::*;
/// #
/// # #[action]
/// # async fn foo(req: &mut Request) -> Result<impl Response, Error> { Ok("foo") }
///
/// fn router(r: &mut Router) {
///     r.scope("api").to(|r| {
///         r.get("foo", foo);
///     });
/// }
/// ```
///
/// You can provide an empty path prefix if you want to group some routes under a middleware
/// pipe or some constraint but don't want to alter their paths.
///
/// ```
/// use reign::router::Router;
/// # use reign::prelude::*;
/// #
/// # #[action]
/// # async fn foo(req: &mut Request) -> Result<impl Response, Error> { Ok("foo") }
///
/// fn router(r: &mut Router) {
///     r.scope("").to(|r| {
///         r.get("foo", foo);
///     });
/// }
/// ```
#[derive(Default)]
pub struct Scope {
    pub(crate) path: Path,
    pub(crate) pipes: Vec<String>,
    pub(crate) router: Router,
    pub(crate) constraint: Option<Arc<Constraint>>,
}

impl Scope {
    pub(crate) fn new<P>(path: P) -> Self
    where
        P: Into<Path>,
    {
        Self {
            path: path.into(),
            ..Default::default()
        }
    }

    /// Define the middleware pipes that run for all the routes under this scope
    ///
    /// # Examples
    ///
    /// ```
    /// use reign::router::{Router, middleware::Runtime};
    /// # use reign::prelude::*;
    /// #
    /// # #[action]
    /// # async fn foo(req: &mut Request) -> Result<impl Response, Error> { Ok("foo") }
    ///
    /// fn router(r: &mut Router) {
    ///     r.pipe("common").add(Runtime::default());
    ///
    ///     r.scope("api").through(&["common"]).to(|r| {
    ///         r.get("foo", foo);
    ///     });
    /// }
    /// ```
    pub fn through<I, S>(&mut self, pipes: I) -> &mut Self
    where
        I: IntoIterator<Item = S>,
        S: ToString,
    {
        self.pipes = pipes.into_iter().map(|x| x.to_string()).collect();
        self
    }

    /// Define the routes that exist under this scope
    ///
    /// Even though you can define a scope without this, it will do nothing and won't affect routing.
    ///
    /// # Examples
    ///
    /// ```
    /// use reign::router::Router;
    /// # use reign::prelude::*;
    /// #
    /// # #[action]
    /// # async fn foo(req: &mut Request) -> Result<impl Response, Error> { Ok("foo") }
    ///
    /// fn router(r: &mut Router) {
    ///     r.scope("api").to(|r| {
    ///         r.get("foo", foo);
    ///     });
    /// }
    /// ```
    pub fn to<R>(&mut self, f: R) -> &mut Self
    where
        R: FnOnce(&mut Router),
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
    ///     r.scope("api").constraint(|req| {
    ///         req.uri().port().is_some() || req.query("bar").is_some()
    ///     }).to(|r| {
    ///         r.get("foo", foo);
    ///     });
    /// }
    /// ```
    pub fn constraint<C>(&mut self, constraint: C) -> &mut Self
    where
        C: Fn(&Request) -> bool + Send + Sync + 'static,
    {
        self.constraint = Some(Arc::new(Box::new(constraint)));
        self
    }

    pub(crate) fn regex(&self) -> (String, Vec<(String, String)>) {
        (self.path.regex(), self.router.regex())
    }

    pub(crate) fn refs(
        &self,
        upper_pipes: Map<&String, &Pipe>,
    ) -> (Option<Arc<Constraint>>, Vec<RouteRef>, Vec<String>) {
        (
            self.constraint.clone(),
            self.router.refs(upper_pipes),
            self.pipes.clone(),
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_through() {
        let mut scope = Scope::new("");
        let scope = scope.through(vec![String::from("one"), String::from("two")]);

        assert_eq!(scope.pipes, vec!["one", "two"]);
    }

    #[test]
    fn test_through_str() {
        let mut scope = Scope::new("");
        let scope = scope.through(vec!["one", "two"]);

        assert_eq!(scope.pipes, vec!["one", "two"]);
    }

    #[test]
    fn test_through_slice() {
        let mut scope = Scope::new("");
        let scope = scope.through(&[String::from("one"), String::from("two")]);

        assert_eq!(scope.pipes, vec!["one", "two"]);
    }

    #[test]
    fn test_through_slice_str() {
        let mut scope = Scope::new("");
        let scope = scope.through(&["one", "two"]);

        assert_eq!(scope.pipes, vec!["one", "two"]);
    }
}
