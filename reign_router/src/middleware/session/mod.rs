//! Contains types needed for session management middleware

use crate::{
    middleware::cookie_parser::CookieParser, Chain, HandleFuture, Middleware, Request, INTERNAL_ERR,
};
use base64::{encode_config, URL_SAFE_NO_PAD};
use bincode::{deserialize, serialize};
use cookie::CookieJar;
use futures::FutureExt;
use hyper::{header::SET_COOKIE, Body, Response};
use log::trace;
use rand::{
    rngs::{adapter::ReseedingRng, OsRng},
    RngCore, SeedableRng,
};
use rand_chacha::ChaChaCore;
use serde::{Deserialize, Serialize};
use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex, PoisonError},
};

pub use cookie::SameSite;

pub mod backends;

/// Represents type that can store session data and is used by the session middleware
pub trait SessionBackend {
    /// Persists a session, either creating a new session or updating an existing session
    fn persist_session<'a>(
        &'a self,
        identifier: &'a str,
        content: &'a [u8],
    ) -> Pin<Box<dyn Future<Output = bool> + Send + 'a>>;

    /// Retrieves a session from the underlying storage
    ///
    /// The returned future will resolve to an `Option<Vec<u8>>` on success, where a value of
    /// `None` indicates that the session is not available for use.
    fn read_session<'a>(
        &'a self,
        identifier: &'a str,
    ) -> Pin<Box<dyn Future<Output = Option<Vec<u8>>> + Send + 'a>>;

    /// Drops a session from the underlying storage
    fn drop_session<'a>(
        &'a self,
        identifier: &'a str,
    ) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>>;
}

pub(crate) enum SessionData<T>
where
    T: Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static,
{
    Dirty(T),
    Clean(T),
    None,
}

/// Manages the session lifecycle
pub struct Session<'a, T, B>
where
    T: Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static,
    B: SessionBackend + Send + Sync,
{
    name: &'a str,
    cookie_secret: Option<&'a str>,
    secure: bool,
    http_only: bool,
    same_site: SameSite,
    path: &'a str,
    domain: Option<&'a str>,
    backend: B,
    rng: Arc<Mutex<ReseedingRng<ChaChaCore, OsRng>>>,
    phantom: std::marker::PhantomData<T>,
}

impl<'a, T, B> Session<'a, T, B>
where
    T: Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static,
    B: SessionBackend + Send + Sync,
{
    /// Instantiates the middleware with type of session data and the backend info
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use reign::router::{Router, middleware::session::{Session, backends::RedisBackend}};
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Serialize, Deserialize)]
    /// pub struct User(String);
    ///
    /// fn router(r: &mut Router) {
    ///     r.pipe("common").add(Session::<, _>::new(RedisBackend::pool(REDIS.pool().clone())));
    /// }
    /// ```
    pub fn new(backend: B) -> Self {
        Self {
            name: "_reign_session",
            cookie_secret: None,
            secure: true,
            http_only: true,
            same_site: SameSite::Lax,
            domain: None,
            path: "/",
            backend,
            rng: Arc::new(Mutex::new(ReseedingRng::new(
                ChaChaCore::from_entropy(),
                32_768,
                OsRng,
            ))),
            phantom: std::marker::PhantomData,
        }
    }

    #[inline]
    pub fn path(mut self, path: &'a str) -> Self {
        self.path = path;
        self
    }

    #[inline]
    pub fn name(mut self, name: &'a str) -> Self {
        self.name = name;
        self
    }

    #[inline]
    pub fn secure(mut self, secure: bool) -> Self {
        self.secure = secure;
        self
    }

    #[inline]
    pub fn http_only(mut self, http_only: bool) -> Self {
        self.http_only = http_only;
        self
    }

    #[inline]
    pub fn domain(mut self, domain: &'a str) -> Self {
        self.domain = Some(domain);
        self
    }

    #[inline]
    pub fn cookie_secret(mut self, secret: &'a str) -> Self {
        self.cookie_secret = Some(secret);
        self
    }

    #[inline]
    pub fn same_site(mut self, same_site: SameSite) -> Self {
        self.same_site = same_site;
        self
    }

    fn cookie_value(&self, value: &str) -> String {
        let mut cookie_value = String::with_capacity(255);

        cookie_value.push_str(&self.name);
        cookie_value.push('=');
        cookie_value.push_str(value);

        if self.secure {
            cookie_value.push_str("; Secure")
        }

        if self.http_only {
            cookie_value.push_str("; HttpOnly")
        }

        match self.same_site {
            SameSite::Strict => cookie_value.push_str("; SameSite=Strict"),
            SameSite::Lax => cookie_value.push_str("; SameSite=Lax"),
            SameSite::None => (),
        }

        if let Some(ref domain) = self.domain {
            cookie_value.push_str("; Domain=");
            cookie_value.push_str(domain);
        }

        cookie_value.push_str("; Path=");
        cookie_value.push_str(&self.path);

        cookie_value
    }

    async fn read_session(&self, req: &mut Request, id: &Option<String>) -> bool {
        if let Some(id) = id {
            trace!("Session id {} found in cookie", id);

            if let Some(data) = self.backend.read_session(id).await {
                if let Ok(bytes) = deserialize::<T>(&data) {
                    req.extensions_mut().insert(SessionData::<T>::Clean(bytes));
                    return true;
                }
            }
        }

        req.extensions_mut().insert(SessionData::<T>::None);
        false
    }

    async fn write_session(
        &self,
        req: &mut Request,
        res: &mut Response<Body>,
        had_data: bool,
        id: &Option<String>,
    ) {
        if let Some(data) = req.extensions_mut().remove::<SessionData<T>>() {
            match data {
                SessionData::Dirty(data) => {
                    if let Ok(bytes) = serialize(&data) {
                        let id = self.random_identifier();
                        let written = self.backend.persist_session(&id, &bytes).await;

                        if written {
                            self.write_cookie(self.cookie_value(&id), res);
                        }
                    }
                }
                SessionData::None if had_data => {
                    self.reset_cookie(res);
                    self.backend
                        .drop_session(id.as_ref().expect(INTERNAL_ERR))
                        .await;
                }
                _ => {}
            }
        }
    }

    fn reset_cookie(&self, res: &mut Response<Body>) {
        self.write_cookie(
            format!(
                "{}; expires=Thu, 01 Jan 1970 00:00:00 GMT; max-age=0",
                self.cookie_value("")
            ),
            res,
        );
    }

    fn write_cookie(&self, value: String, res: &mut Response<Body>) {
        res.headers_mut()
            .append(SET_COOKIE, value.parse().expect(INTERNAL_ERR));
    }

    fn random_identifier(&self) -> String {
        let mut bytes = [0u8; 64];

        match self.rng.lock() {
            Ok(mut rng) => rng.fill_bytes(&mut bytes),
            Err(PoisonError { .. }) => unreachable!("identifier_rng lock poisoned. Rng panicked?"),
        };

        encode_config(&bytes[..], URL_SAFE_NO_PAD)
    }
}

impl<'a, T, B> Middleware for Session<'a, T, B>
where
    T: Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static,
    B: SessionBackend + Send + Sync,
{
    fn handle<'m>(&'m self, req: &'m mut Request, chain: Chain<'m>) -> HandleFuture<'m> {
        let cookies = req
            .extensions()
            .get::<CookieJar>()
            .cloned()
            .unwrap_or_else(|| {
                let mut parser = CookieParser::new();

                if let Some(secret) = self.cookie_secret {
                    parser = parser.secret(secret);
                }

                parser.parse(req)
            });

        let id = cookies.get(self.name).map(|x| x.value().to_string());

        async move {
            let had_data = self.read_session(req, &id).await;

            let mut response = chain.run(req).await?;

            self.write_session(req, &mut response, had_data, &id).await;

            Ok(response)
        }
        .boxed()
    }
}
