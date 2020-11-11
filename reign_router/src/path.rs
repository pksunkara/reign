// use percent_encoding::utf8_percent_encode;

#[derive(Debug, Clone)]
enum PathPart {
    Static(String),
    Param(String),
    ParamOpt(String),
    ParamRegex(String, String),
    ParamOptRegex(String, String),
}

/// Path that is specified for a route in the router definition
///
/// # Examples
///
/// ```
/// use reign::router::{Router, Path};
/// # use reign::prelude::*;
/// #
/// # #[action]
/// # async fn foo_show(req: &mut Request) -> Result<impl Response, Error> { Ok("foo") }
/// #
/// # #[action]
/// # async fn foo_edit(req: &mut Request) -> Result<impl Response, Error> { Ok("foo") }
///
/// fn router(r: &mut Router) {
///     r.get(Path::new().param("id"), foo_show);
///     r.post(Path::new().param_opt("id"), foo_edit);
/// }
/// ```
#[derive(Debug, Default, Clone)]
pub struct Path {
    parts: Vec<PathPart>,
}

impl Path {
    /// Create a new empty path that has no path segments at all
    ///
    /// # Examples
    ///
    /// ```
    /// use reign::router::{Router, Path};
    /// # use reign::prelude::*;
    /// #
    /// # #[action]
    /// # async fn foo(req: &mut Request) -> Result<impl Response, Error> { Ok("foo") }
    ///
    /// fn router(r: &mut Router) {
    ///     r.get(Path::new(), foo);
    /// }
    /// ```
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add the given string as a static path segment to the path
    ///
    /// # Examples
    ///
    /// ```
    /// use reign::router::{Router, Path};
    /// # use reign::prelude::*;
    /// #
    /// # #[action]
    /// # async fn foo(req: &mut Request) -> Result<impl Response, Error> { Ok("foo") }
    ///
    /// fn router(r: &mut Router) {
    ///     r.get(Path::new().path("foo"), foo);
    /// }
    /// ```
    pub fn path<S>(mut self, value: S) -> Self
    where
        S: Into<String>,
    {
        // TODO: router:url: /, ?, # should be encoded
        let value = value.into();

        if !value.is_empty() {
            self.parts.push(PathPart::Static(value));
        }

        self
    }

    /// Add a required path parameter with the given name to the path
    ///
    /// # Examples
    ///
    /// ```
    /// use reign::router::{Router, Path};
    /// # use reign::prelude::*;
    /// #
    /// # #[action]
    /// # async fn foo(req: &mut Request) -> Result<impl Response, Error> { Ok("foo") }
    ///
    /// fn router(r: &mut Router) {
    ///     r.get(Path::new().param("id"), foo);
    /// }
    /// ```
    pub fn param<S>(mut self, name: S) -> Self
    where
        S: Into<String>,
    {
        self.parts.push(PathPart::Param(name.into()));
        self
    }

    /// Add an optional path parameter with the given name to the path
    ///
    /// # Examples
    ///
    /// ```
    /// use reign::router::{Router, Path};
    /// # use reign::prelude::*;
    /// #
    /// # #[action]
    /// # async fn foo(req: &mut Request) -> Result<impl Response, Error> { Ok("foo") }
    ///
    /// fn router(r: &mut Router) {
    ///     r.get(Path::new().param_opt("id"), foo);
    /// }
    /// ```
    pub fn param_opt<S>(mut self, name: S) -> Self
    where
        S: Into<String>,
    {
        self.parts.push(PathPart::ParamOpt(name.into()));
        self
    }

    /// Add a required regex path parameter with the given name and regex pattern to the path
    ///
    /// # Examples
    ///
    /// ```
    /// use reign::router::{Router, Path};
    /// # use reign::prelude::*;
    /// #
    /// # #[action]
    /// # async fn foo(req: &mut Request) -> Result<impl Response, Error> { Ok("foo") }
    ///
    /// fn router(r: &mut Router) {
    ///     r.get(Path::new().param_regex("id", "[0-9]+"), foo);
    /// }
    /// ```
    ///
    /// You can also add a required glob path parameter by defining the regex to contain `/`
    ///
    /// ```
    /// use reign::router::{Router, Path};
    /// # use reign::prelude::*;
    /// #
    /// # #[action]
    /// # async fn foo(req: &mut Request) -> Result<impl Response, Error> { Ok("foo") }
    ///
    /// fn router(r: &mut Router) {
    ///     r.get(Path::new().param_regex("id", ".+"), foo);
    /// }
    /// ```
    pub fn param_regex<S, R>(mut self, name: S, regex: R) -> Self
    where
        S: Into<String>,
        R: Into<String>,
    {
        self.parts
            .push(PathPart::ParamRegex(name.into(), regex.into()));
        self
    }

    /// Add an optional regex path parameter with the given name and regex pattern to the path
    ///
    /// # Examples
    ///
    /// ```
    /// use reign::router::{Router, Path};
    /// # use reign::prelude::*;
    /// #
    /// # #[action]
    /// # async fn foo(req: &mut Request) -> Result<impl Response, Error> { Ok("foo") }
    ///
    /// fn router(r: &mut Router) {
    ///     r.get(Path::new().param_opt_regex("id", "[0-9]+"), foo);
    /// }
    /// ```
    ///
    /// You can also add an optional glob path parameter by defining the regex to contain `/`
    ///
    /// ```
    /// use reign::router::{Router, Path};
    /// # use reign::prelude::*;
    /// #
    /// # #[action]
    /// # async fn foo(req: &mut Request) -> Result<impl Response, Error> { Ok("foo") }
    ///
    /// fn router(r: &mut Router) {
    ///     r.get(Path::new().param_opt_regex("id", ".+"), foo);
    /// }
    /// ```
    pub fn param_opt_regex<S, R>(mut self, name: S, regex: R) -> Self
    where
        S: Into<String>,
        R: Into<String>,
    {
        self.parts
            .push(PathPart::ParamOptRegex(name.into(), regex.into()));
        self
    }

    pub(crate) fn regex(&self) -> String {
        let mut regex = vec![];

        for part in &self.parts {
            match part {
                PathPart::Static(p) => regex.push(format!("/{}", p)),
                PathPart::Param(p) => regex.push(format!("/(?P<{}>[^/]+)", p)),
                PathPart::ParamOpt(p) => regex.push(format!("(/(?P<{}>[^/]+))?", p)),
                PathPart::ParamRegex(p, r) => regex.push(format!("/(?P<{}>{})", p, r)),
                PathPart::ParamOptRegex(p, r) => regex.push(format!("(/(?P<{}>{}))?", p, r)),
            }
        }

        regex.join("")
    }
}

impl<'a> Into<Path> for &'a str {
    fn into(self) -> Path {
        Path::new().path(self)
    }
}

impl Into<Path> for String {
    fn into(self) -> Path {
        Path::new().path(self)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_regex_param_static() {
        let p = Path::new().path("foo").path("bar");
        assert_eq!(p.regex(), "/foo/bar");
    }
}
