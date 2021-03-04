use crate::{
    futures::FutureExt,
    hyper::{header::CONTENT_TYPE, StatusCode},
    Chain, Error, HandleFuture, Middleware, Request,
};

use mime::{Mime, Name, FORM_DATA, JSON, WWW_FORM_URLENCODED};

/// Only allows certain content-type headers for request bodies
///
/// By default, JSON and URL encoded are allowed.
#[derive(Debug, Clone)]
pub struct ContentType<'a> {
    subtypes: Vec<&'a str>,
}

impl<'a> ContentType<'a> {
    pub fn new(subtypes: Vec<&'a str>) -> Self {
        Self { subtypes }
    }

    pub fn default() -> Self {
        Self::empty().json().form()
    }

    pub fn empty() -> Self {
        Self::new(vec![])
    }

    pub fn json(mut self) -> Self {
        self.subtypes.push(JSON.as_str());
        self
    }

    pub fn form(mut self) -> Self {
        self.subtypes.push(WWW_FORM_URLENCODED.as_str());
        self
    }

    pub fn multipart(mut self) -> Self {
        self.subtypes.push(FORM_DATA.as_str());
        self
    }

    fn allow(&self, val: Name) -> bool {
        self.subtypes.iter().any(|&x| x == val.as_str())
    }
}

impl<'a> Middleware for ContentType<'a> {
    fn handle<'m>(&'m self, req: &'m mut Request, chain: Chain<'m>) -> HandleFuture<'m> {
        match req.headers().get(CONTENT_TYPE) {
            Some(content_type) => {
                if let Ok(content_type) = content_type.to_str() {
                    if let Ok(val) = content_type.parse::<Mime>() {
                        if self.allow(val.subtype()) {
                            return chain.run(req);
                        }

                        if let Some(suffix) = val.suffix() {
                            if self.allow(suffix) {
                                return chain.run(req);
                            }
                        }
                    }
                }
            }
            None => {
                return chain.run(req);
            }
        };

        async { Err(Error::Status(StatusCode::UNSUPPORTED_MEDIA_TYPE)) }.boxed()
    }
}
