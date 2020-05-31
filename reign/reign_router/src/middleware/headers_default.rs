use futures::prelude::*;
use std::pin::Pin;

#[derive(Debug, Clone)]
pub struct HeadersDefault {
    headers: Vec<(&'static str, &'static str)>,
}

impl HeadersDefault {
    pub fn new(headers: Vec<(&'static str, &'static str)>) -> Self {
        Self { headers }
    }

    pub fn default() -> Self {
        Self::empty().add("x-powered-by", "reign")
    }

    pub fn empty() -> Self {
        Self::new(vec![])
    }

    pub fn add(mut self, name: &'static str, value: &'static str) -> Self {
        if name.to_lowercase() != name {
            panic!("Only lowercase headers are allowed");
        }

        self.headers.push((name, value));
        self
    }
}

#[cfg(test)]
mod test {
    use super::HeadersDefault;

    #[test]
    #[should_panic]
    fn test_with_uppercase() {
        HeadersDefault::empty().add("X-Version", "0.1");
    }
}
