// use percent_encoding::utf8_percent_encode;

#[derive(Debug, Clone)]
enum PathPart<'a> {
    Static(&'a str),
    Param(&'a str),
    ParamOpt(&'a str),
    ParamRegex(&'a str, &'a str),
    ParamOptRegex(&'a str, &'a str),
}

#[derive(Debug, Default, Clone)]
pub struct Path<'a> {
    parts: Vec<PathPart<'a>>,
}

impl<'a> Path<'a> {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn path(mut self, value: &'a str) -> Self {
        // TODO:(router:percent) /, ?, # should be encoded
        if !value.is_empty() {
            self.parts.push(PathPart::Static(value));
        }

        self
    }

    pub fn param(mut self, name: &'a str) -> Self {
        self.parts.push(PathPart::Param(name));
        self
    }

    pub fn param_opt(mut self, name: &'a str) -> Self {
        self.parts.push(PathPart::ParamOpt(name));
        self
    }

    pub fn param_regex(mut self, name: &'a str, regex: &'a str) -> Self {
        self.parts.push(PathPart::ParamRegex(name, regex));
        self
    }

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
