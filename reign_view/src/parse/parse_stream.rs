use crate::parse::{Error, Parse};
use regex::Regex;

#[derive(Debug)]
pub struct ParseStream {
    pub content: String,
    pub cursor: usize,
}

impl ParseStream {
    pub fn new(content: String) -> Self {
        ParseStream { content, cursor: 0 }
    }

    pub fn error(&self, msg: &str) -> Error {
        Error {
            content: self.content.clone(),
            cursor: self.cursor,
            message: msg.to_string(),
        }
    }

    pub fn parse<T>(&mut self) -> Result<T, Error>
    where
        T: Parse,
    {
        T::parse(self)
    }

    pub fn is_match(&self, r: &str) -> bool {
        Regex::new(&format!("^{}", r))
            .unwrap()
            .is_match(self.content.get(self.cursor..).unwrap())
    }

    pub fn matched(&mut self, r: &str) -> Result<String, Error> {
        let reg = Regex::new(&format!("^{}", r)).unwrap();
        let mat = reg.find(self.content.get(self.cursor..).unwrap());

        if mat.is_none() {
            return Err(self.error(&format!("unable to match `{}`", r)));
        }

        let mat_end = self.cursor + mat.unwrap().end();
        let sub_string = self.content.get(self.cursor..mat_end);

        if sub_string.is_none() {
            return Err(self.error("out of bounds"));
        }

        self.cursor = mat_end;
        Ok(sub_string.unwrap().to_string())
    }

    pub fn capture(&mut self, r: &str, index: usize) -> Result<String, Error> {
        let reg = Regex::new(&format!("^{}", r)).unwrap();
        let cap = reg.captures(self.content.get(self.cursor..).unwrap());

        if cap.is_none() {
            return Err(self.error(&format!("unable to match `{}`", r)));
        }

        let captures = cap.unwrap();
        let val = captures.get(index);

        if val.is_none() {
            return Err(self.error(&format!("unable to get capture group {} in `{}`", index, r)));
        }

        self.cursor += captures.get(0).unwrap().as_str().len();
        Ok(val.unwrap().as_str().to_string())
    }

    pub fn peek(&self, sub: &str) -> bool {
        let sub_end = self.cursor + sub.len();
        let sub_string = self.content.get(self.cursor..sub_end);

        if sub_string.is_none() {
            return false;
        }

        sub_string.unwrap() == sub
    }

    pub fn step(&mut self, sub: &str) -> Result<String, Error> {
        let sub_end = self.cursor + sub.len();
        let sub_string = self.content.get(self.cursor..sub_end);

        if sub_string.is_none() {
            return Err(self.error(&format!("out of bounds when trying to find `{}`", sub)));
        }

        if sub_string.unwrap() != sub {
            return Err(self.error(&format!("expected `{}`", sub)));
        }

        self.cursor = sub_end;
        Ok(sub_string.unwrap().to_string())
    }

    pub fn seek(&self, sub: &str) -> Result<usize, Error> {
        let index = self.content.get(self.cursor..).unwrap().find(sub);

        if index.is_none() {
            return Err(self.error(&format!("expected `{}`", sub)));
        }

        Ok(self.cursor + index.unwrap())
    }

    pub fn until(&mut self, sub: &str, consume: bool) -> Result<String, Error> {
        let index = self.seek(sub)?;
        let sub_string = self.content.get(self.cursor..index);

        self.cursor = index;

        if consume {
            self.cursor += sub.len();
        }

        Ok(sub_string.unwrap().to_string())
    }

    pub fn skip_spaces(&mut self) -> Result<(), Error> {
        self.matched("\\s*")?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::ParseStream;

    #[test]
    fn test_is_match() {
        let ps = ParseStream {
            content: "Hello World".to_string(),
            cursor: 1,
        };

        assert!(ps.is_match("[a-z]+"));
        assert!(!ps.is_match("[A-Z][a-z]+"));
    }

    #[test]
    fn test_matched() {
        let mut ps = ParseStream {
            content: "Hello World".to_string(),
            cursor: 2,
        };

        let val = ps.matched("[a-z]+").unwrap();

        assert_eq!(ps.cursor, 5);
        assert_eq!(val, "llo".to_string());
    }

    #[test]
    fn test_matched_error() {
        let mut ps = ParseStream {
            content: "Hello World".to_string(),
            cursor: 1,
        };

        let err = ps.matched("[A-Z]+").unwrap_err();

        assert_eq!(ps.cursor, 1);
        assert_eq!(err.cursor, 1);
        assert_eq!(err.message, "unable to match `[A-Z]+`".to_string());
    }

    #[test]
    fn test_capture() {
        let mut ps = ParseStream {
            content: "Hello World".to_string(),
            cursor: 3,
        };

        let val = ps.capture("([a-z])([a-z])", 2).unwrap();

        assert_eq!(ps.cursor, 5);
        assert_eq!(val, "o".to_string());
    }

    #[test]
    fn test_capture_error() {
        let mut ps = ParseStream {
            content: "Hello World".to_string(),
            cursor: 1,
        };

        let err = ps.capture("[A-Z]+", 1).unwrap_err();

        assert_eq!(ps.cursor, 1);
        assert_eq!(err.cursor, 1);
        assert_eq!(err.message, "unable to match `[A-Z]+`".to_string());
    }

    #[test]
    fn test_capture_number_error() {
        let mut ps = ParseStream {
            content: "Hello World".to_string(),
            cursor: 1,
        };

        let err = ps.capture("([a-z])([a-z])", 3).unwrap_err();

        assert_eq!(ps.cursor, 1);
        assert_eq!(err.cursor, 1);
        assert_eq!(
            err.message,
            "unable to get capture group 3 in `([a-z])([a-z])`".to_string()
        );
    }

    #[test]
    fn test_peek() {
        let ps = ParseStream {
            content: "Hello".to_string(),
            cursor: 1,
        };

        assert!(ps.peek("ello"));
        assert!(!ps.peek("Hello"));
    }

    #[test]
    fn test_step() {
        let mut ps = ParseStream {
            content: "Hello".to_string(),
            cursor: 1,
        };

        let val = ps.step("el").unwrap();

        assert_eq!(ps.cursor, 3);
        assert_eq!(val, "el");
    }

    #[test]
    fn test_step_error() {
        let mut ps = ParseStream {
            content: "Hello".to_string(),
            cursor: 1,
        };

        let err = ps.step("Hel").unwrap_err();

        assert_eq!(ps.cursor, 1);
        assert_eq!(err.cursor, 1);
        assert_eq!(err.message, "expected `Hel`".to_string())
    }

    #[test]
    fn test_step_bounds_error() {
        let mut ps = ParseStream {
            content: "Hello".to_string(),
            cursor: 1,
        };

        let err = ps.step("Hello").unwrap_err();

        assert_eq!(ps.cursor, 1);
        assert_eq!(err.cursor, 1);
        assert_eq!(
            err.message,
            "out of bounds when trying to find `Hello`".to_string(),
        )
    }

    #[test]
    fn test_seek() {
        let ps = ParseStream {
            content: "Hello".to_string(),
            cursor: 1,
        };

        let index = ps.seek("lo").unwrap();

        assert_eq!(ps.cursor, 1);
        assert_eq!(index, 3);
    }

    #[test]
    fn test_seek_error() {
        let ps = ParseStream {
            content: "Hello".to_string(),
            cursor: 1,
        };

        let err = ps.seek("H").unwrap_err();

        assert_eq!(ps.cursor, 1);
        assert_eq!(err.cursor, 1);
        assert_eq!(err.message, "expected `H`".to_string())
    }

    #[test]
    fn test_until() {
        let mut ps = ParseStream {
            content: "Hello World".to_string(),
            cursor: 1,
        };

        let val = ps.until("lo", true).unwrap();

        assert_eq!(ps.cursor, 5);
        assert_eq!(val, "el".to_string());
    }

    #[test]
    fn test_until_non_consume() {
        let mut ps = ParseStream {
            content: "Hello World".to_string(),
            cursor: 1,
        };

        let val = ps.until("lo", false).unwrap();

        assert_eq!(ps.cursor, 3);
        assert_eq!(val, "el".to_string());
    }

    #[test]
    fn test_until_error() {
        let mut ps = ParseStream {
            content: "Hello World".to_string(),
            cursor: 1,
        };

        let err = ps.until("H", true).unwrap_err();

        assert_eq!(ps.cursor, 1);
        assert_eq!(err.cursor, 1);
        assert_eq!(err.message, "expected `H`".to_string())
    }
}
