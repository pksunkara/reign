use crate::{
    futures::FutureExt,
    hyper::header::{HeaderName, HeaderValue},
    Chain, HandleFuture, Middleware, Request,
};

/// Adds some default headers to all responses.
#[derive(Debug, Clone)]
pub struct HeadersDefault {
    headers: Vec<(HeaderName, HeaderValue)>,
}

impl HeadersDefault {
    pub fn new(headers: &[(&str, &str)]) -> Self {
        let mut ret = Self { headers: vec![] };

        for (name, value) in headers {
            ret = ret.add(name, value);
        }

        ret
    }

    pub fn default() -> Self {
        Self::empty().add("x-powered-by", "reign")
    }

    pub fn empty() -> Self {
        Self::new(&[])
    }

    pub fn add(mut self, name: &str, value: &str) -> Self {
        if name.to_lowercase() != name {
            panic!("Only lowercase headers are allowed");
        }

        self.headers.push((
            HeaderName::from_lowercase(name.as_bytes()).unwrap(),
            HeaderValue::from_str(value).unwrap(),
        ));
        self
    }
}

impl Middleware for HeadersDefault {
    fn handle<'m>(&'m self, req: &'m mut Request, chain: Chain<'m>) -> HandleFuture<'m> {
        async move {
            let mut response = chain.run(req).await?;

            for (name, value) in &self.headers {
                response.headers_mut().insert(name.clone(), value.clone());
            }

            Ok(response)
        }
        .boxed()
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

    #[test]
    #[should_panic]
    fn test_new_with_uppercase() {
        HeadersDefault::new(&[("X-Version", "0.1")]);
    }
}
