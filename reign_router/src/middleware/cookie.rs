//! Contains types needed for cookie parsing middleware

use crate::{Chain, HandleFuture, Middleware, Request};

use cookie_r::Cookie;
use hyper::header::{HeaderValue, COOKIE};

pub use cookie_r::CookieJar;

/// Parses the cookie and adds a CookieJar to the request storage
#[derive(Default)]
pub struct CookieParser {}

impl CookieParser {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    pub(crate) fn parse(&self, req: &mut Request) -> CookieJar {
        req.headers()
            .get_all(COOKIE)
            .iter()
            .flat_map(HeaderValue::to_str)
            .flat_map(|x| x.split("; "))
            .flat_map(|x| Cookie::parse(x.to_owned()))
            .fold(CookieJar::new(), |mut jar, cookie| {
                jar.add_original(cookie);
                jar
            })
    }
}

impl Middleware for CookieParser {
    fn handle<'m>(&'m self, req: &'m mut Request, chain: Chain<'m>) -> HandleFuture<'m> {
        let jar = self.parse(req);
        req.extensions_mut().insert(jar);

        chain.run(req)
    }
}
