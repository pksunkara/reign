use crate::{Chain, HandleFuture, Middleware, Request};
use cookie::Cookie;
use hyper::header::{HeaderValue, COOKIE};

pub use cookie::CookieJar;

#[derive(Default)]
pub struct CookieParser<'a> {
    secret: Option<&'a str>,
}

// TODO:(router:cookie) private
impl<'a> CookieParser<'a> {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn secret(mut self, secret: &'a str) -> Self {
        self.secret = Some(secret);
        self
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

impl<'a> Middleware for CookieParser<'a> {
    fn handle<'m>(&'m self, req: &'m mut Request, chain: Chain<'m>) -> HandleFuture<'m> {
        let jar = self.parse(req);
        req.extensions.insert(jar);

        chain.run(req)
    }
}
