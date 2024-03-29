// use percent_encoding::utf8_percent_encode;

#[derive(Debug, Clone)]
enum PathPart {
    Static(String),
    Param(String),
    ParamOpt(String),
    ParamRegex(String, String),
    ParamOptRegex(String, String),
}

/// Path that is specified for a route in the router definition.
///
/// # Examples
///
/// ```
/// use reign::router::{Path, Router};
/// # use reign::prelude::*;
/// #
/// # async fn foo_show(req: &mut Request) -> Result<impl Response, Error> { Ok("foo") }
/// #
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
    /// Create a new empty path that has no path segments at all.
    ///
    /// # Examples
    ///
    /// ```
    /// use reign::router::{Path, Router};
    /// # use reign::prelude::*;
    /// #
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

    /// Add the given string as a static path segment to the path.
    ///
    /// # Examples
    ///
    /// ```
    /// use reign::router::{Path, Router};
    /// # use reign::prelude::*;
    /// #
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
        let value = value.into();

        if !value.is_empty() {
            self.parts.push(PathPart::Static(value));
        }

        self
    }

    /// Add a required path parameter with the given name to the path.
    ///
    /// # Examples
    ///
    /// ```
    /// use reign::router::{Path, Router};
    /// # use reign::prelude::*;
    /// #
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

    /// Add an optional path parameter with the given name to the path.
    ///
    /// # Examples
    ///
    /// ```
    /// use reign::router::{Path, Router};
    /// # use reign::prelude::*;
    /// #
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

    /// Add a required regex path parameter with the given name and regex pattern to the path.
    ///
    /// # Examples
    ///
    /// ```
    /// use reign::router::{Path, Router};
    /// # use reign::prelude::*;
    /// #
    /// # async fn foo(req: &mut Request) -> Result<impl Response, Error> { Ok("foo") }
    ///
    /// fn router(r: &mut Router) {
    ///     r.get(Path::new().param_regex("id", "[0-9]+"), foo);
    /// }
    /// ```
    ///
    /// You can also add a required glob path parameter by defining the regex to contain `/`.
    ///
    /// ```
    /// use reign::router::{Path, Router};
    /// # use reign::prelude::*;
    /// #
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

    /// Add an optional regex path parameter with the given name and regex pattern to the path.
    ///
    /// # Examples
    ///
    /// ```
    /// use reign::router::{Path, Router};
    /// # use reign::prelude::*;
    /// #
    /// # async fn foo(req: &mut Request) -> Result<impl Response, Error> { Ok("foo") }
    ///
    /// fn router(r: &mut Router) {
    ///     r.get(Path::new().param_opt_regex("id", "[0-9]+"), foo);
    /// }
    /// ```
    ///
    /// You can also add an optional glob path parameter by defining the regex to contain `/`.
    ///
    /// ```
    /// use reign::router::{Path, Router};
    /// # use reign::prelude::*;
    /// #
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

/// Helper for defining a [reign_router] Path.
///
/// # Examples
///
/// ```
/// use reign::{prelude::*, router::{path as p, Router}};
/// #
/// # async fn foobar(req: &mut Request) -> Result<impl Response, Error> { Ok("foobar") }
/// #
/// # async fn number(req: &mut Request) -> Result<impl Response, Error> { Ok("number") }
/// #
/// # async fn tree(req: &mut Request) -> Result<impl Response, Error> { Ok("tree") }
///
/// fn router(r: &mut Router) {
///     // Required param
///     r.get(p!("foo" / id / "bar"), foobar);
///
///     // Optional param
///     r.get(p!("foo" / id?), foobar);
///
///     // Regex param
///     r.get(p!("number" / id @ "[0-9]+"), number);
///
///     // Optional Regex param
///     r.get(p!("number" / id? @ "[0-9]+"), number);
///
///     // Glob param
///     r.get(p!("tree" / id*), tree);
///
///     // Optional Glob param
///     r.get(p!("tree" / id*?), tree);
/// }
/// ```
#[macro_export]
macro_rules! path {
    (@expr $e:expr,) => { $e };
    (@expr $e:expr, / $part:literal $($tail:tt)*) => {
        $crate::path!(@expr $e.path($part), $($tail)*);
    };
    (@expr $e:expr, / $part:ident @ $regex:literal $($tail:tt)*) => {
        $crate::path!(@expr $e.param_regex(stringify!($part), $regex), $($tail)*);
    };
    (@expr $e:expr, / $part:ident ? @ $regex:literal $($tail:tt)*) => {
        $crate::path!(@expr $e.param_opt_regex(stringify!($part), $regex), $($tail)*);
    };
    (@expr $e:expr, / $part:ident * ? $($tail:tt)*) => {
        $crate::path!(@expr $e.param_opt_regex(stringify!($part), ".+"), $($tail)*);
    };
    (@expr $e:expr, / $part:ident * $($tail:tt)*) => {
        $crate::path!(@expr $e.param_regex(stringify!($part), ".+"), $($tail)*);
    };
    (@expr $e:expr, / $part:ident ? $($tail:tt)*) => {
        $crate::path!(@expr $e.param_opt(stringify!($part)), $($tail)*);
    };
    (@expr $e:expr, / $part:ident $($tail:tt)*) => {
        $crate::path!(@expr $e.param(stringify!($part)), $($tail)*);
    };
    ($($tail:tt)+) => {
        $crate::path!(@expr $crate::Path::new(), / $($tail)*);
    };
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
