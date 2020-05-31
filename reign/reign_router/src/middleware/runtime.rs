use crate::{
    futures::FutureExt,
    hyper::header::{HeaderName, HeaderValue},
    Chain, HandleFuture, Middleware, Request, INTERNAL_ERR,
};
use chrono::prelude::Utc;

fn dur_to_string(i: i64) -> String {
    if i < 1000 {
        format!("{}us", i)
    } else if i < 1_000_000 {
        format!("{:.2}ms", (i as f32) / 1000.0)
    } else {
        format!("{:.2}s", (i as f32) / 1_000_000.0)
    }
}

#[derive(Debug, Clone)]
pub struct Runtime {
    header: HeaderName,
}

impl Runtime {
    pub fn new(header: &str) -> Self {
        Self {
            header: HeaderName::from_lowercase(header.as_bytes()).unwrap(),
        }
    }

    pub fn default() -> Self {
        Self::new("x-runtime")
    }
}

impl Middleware for Runtime {
    fn handle<'m>(&'m self, req: &'m mut Request, chain: Chain<'m>) -> HandleFuture<'m> {
        async move {
            let start = Utc::now();
            let mut response = chain.run(req).await?;
            let duration = Utc::now().signed_duration_since(start).num_microseconds();

            if let Some(dur) = duration {
                response.headers_mut().insert(
                    self.header.clone(),
                    HeaderValue::from_str(&dur_to_string(dur)).expect(INTERNAL_ERR),
                );
            }

            Ok(response)
        }
        .boxed()
    }
}

#[cfg(test)]
mod test {
    use super::{dur_to_string, Runtime};

    #[test]
    fn test_dur_to_string_micro_seconds() {
        assert_eq!(&dur_to_string(193), "193us");
    }

    #[test]
    fn test_dur_to_string_milli_seconds() {
        assert_eq!(&dur_to_string(2940), "2.94ms");
    }

    #[test]
    fn test_dur_to_string_seconds() {
        assert_eq!(&dur_to_string(3495773), "3.50s");
    }

    #[test]
    #[should_panic]
    fn test_runtime_with_uppercase() {
        Runtime::new("X-Runtime");
    }
}
