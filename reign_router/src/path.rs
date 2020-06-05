// use percent_encoding::utf8_percent_encode;

#[derive(Debug, Clone)]
enum PathPart<'a> {
    Static(&'a str),
    Param(&'a str),
    ParamOpt(&'a str),
    ParamRegex(&'a str, &'a str),
    ParamOptRegex(&'a str, &'a str),
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
pub struct Path<'a> {
    parts: Vec<PathPart<'a>>,
}

impl<'a> Path<'a> {
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
    pub fn path(mut self, value: &'a str) -> Self {
        // TODO:(router:percent) /, ?, # should be encoded
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
    pub fn param(mut self, name: &'a str) -> Self {
        self.parts.push(PathPart::Param(name));
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
    pub fn param_opt(mut self, name: &'a str) -> Self {
        self.parts.push(PathPart::ParamOpt(name));
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
    pub fn param_regex(mut self, name: &'a str, regex: &'a str) -> Self {
        self.parts.push(PathPart::ParamRegex(name, regex));
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
    pub fn param_opt_regex(mut self, name: &'a str, regex: &'a str) -> Self {
        self.parts.push(PathPart::ParamOptRegex(name, regex));
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

impl<'a> Into<Path<'a>> for &'a str {
    fn into(self) -> Path<'a> {
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
