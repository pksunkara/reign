#[cfg(feature = "session")]
use crate::middleware::session::SessionData;
use crate::{Error, ParamError};
use hyper::{
    body::{to_bytes, Bytes},
    http::{request::Parts, Extensions},
    Body, HeaderMap, Method, Request as HyperRequest, Uri, Version,
};
#[cfg(feature = "session")]
use serde::{Deserialize, Serialize};
use std::{collections::HashMap as Map, net::SocketAddr};
use url::form_urlencoded::parse;

/// Request denotes the incoming request to the server and also acts as a state
///
/// # Examples
///
/// ```
/// use reign::prelude::*;
///
/// #[action]
/// async fn foo(req: &mut Request) -> Result<impl Response, Error> {
///     Ok(req.uri().host().unwrap_or("").to_string())
/// }
/// ```
#[derive(Debug)]
pub struct Request {
    parts: Parts,
    ip: SocketAddr,
    pub(crate) params: Map<String, String>,
    pub(crate) query: Map<String, String>,
}

impl Request {
    pub(crate) fn new(ip: SocketAddr, req: HyperRequest<Body>) -> Self {
        let (parts, body) = req.into_parts();

        let mut ret = Self {
            parts,
            ip,
            params: Map::new(),
            query: Map::new(),
        };

        if let Some(query) = ret
            .uri()
            .query()
            .map(|v| parse(v.as_bytes()).into_owned().collect())
        {
            ret.query = query;
        }

        ret.parts.extensions.insert(body);
        ret
    }

    /// Returns a reference to the associated remote IP socket address
    #[inline]
    pub fn ip(&self) -> &SocketAddr {
        &self.ip
    }

    /// Returns a reference to the associated HTTP Method.
    ///
    /// # Examples
    ///
    /// ```
    /// use reign::{prelude::*, router::hyper::Method};
    ///
    /// #[action]
    /// async fn foo(req: &mut Request) -> Result<impl Response, Error> {
    ///     Ok((*req.method() == Method::GET).to_string())
    /// }
    /// ```
    #[inline]
    pub fn method(&self) -> &Method {
        &self.parts.method
    }

    /// Returns a reference to the associated version
    ///
    /// # Examples
    ///
    /// ```
    /// use reign::{prelude::*, router::hyper::Version};
    ///
    /// #[action]
    /// async fn foo(req: &mut Request) -> Result<impl Response, Error> {
    ///     Ok((*req.version() == Version::HTTP_2).to_string())
    /// }
    /// ```
    #[inline]
    pub fn version(&self) -> &Version {
        &self.parts.version
    }

    /// Returns a reference to the associated URI.
    ///
    /// # Examples
    ///
    /// ```
    /// use reign::prelude::*;
    ///
    /// #[action]
    /// async fn foo(req: &mut Request) -> Result<impl Response, Error> {
    ///     Ok(req.uri().path().to_string())
    /// }
    /// ```
    #[inline]
    pub fn uri(&self) -> &Uri {
        &self.parts.uri
    }

    /// Returns a reference to the associated HeaderMap.
    ///
    /// # Examples
    ///
    /// ```
    /// use reign::prelude::*;
    ///
    /// #[action]
    /// async fn foo(req: &mut Request) -> Result<impl Response, Error> {
    ///     if let Some(val) = req.headers().get("x-version") {
    ///         Ok(val.to_str()?.to_string())
    ///     } else {
    ///         Ok("No value".into())
    ///     }
    /// }
    /// ```
    #[inline]
    pub fn headers(&self) -> &HeaderMap {
        &self.parts.headers
    }

    /// Returns a reference to the underlying any-type storage
    ///
    /// # Examples
    ///
    /// ```
    /// use reign::prelude::*;
    ///
    /// #[derive(Clone)]
    /// struct Custom(String);
    ///
    /// #[action]
    /// async fn foo(req: &mut Request) -> Result<impl Response, Error> {
    ///     if let Some(val) = req.extensions().get::<Custom>() {
    ///         Ok(val.clone().0)
    ///     } else {
    ///         Ok("No data".into())
    ///     }
    /// }
    /// ```
    #[inline]
    pub fn extensions(&self) -> &Extensions {
        &self.parts.extensions
    }

    /// Returns a mutable reference to the underlying any-type storage
    ///
    /// # Examples
    ///
    /// ```
    /// use reign::{prelude::*, router::Middleware};
    ///
    /// struct Custom(String);
    ///
    /// #[action]
    /// async fn foo(req: &mut Request) -> Result<impl Response, Error> {
    ///     if let Some(val) = req.extensions_mut().remove::<Custom>() {
    ///         Ok(val.0)
    ///     } else {
    ///         Ok("No data".into())
    ///     }
    /// }
    /// ```
    #[inline]
    pub fn extensions_mut(&mut self) -> &mut Extensions {
        &mut self.parts.extensions
    }

    /// Retrieve the Request body.
    ///
    /// This consumes the body from the request and it will not be available for
    /// any other handlers after this.
    ///
    /// # Examples
    ///
    /// ```
    /// use reign::prelude::*;
    /// use std::str::from_utf8;
    ///
    /// #[action]
    /// async fn foo(req: &mut Request) -> Result<impl Response, Error> {
    ///     if let Some(bytes) = req.body().await? {
    ///         Ok(from_utf8(&bytes)?.to_string())
    ///     } else {
    ///         Ok("No body".into())
    ///     }
    /// }
    /// ```
    pub async fn body(&mut self) -> Result<Option<Bytes>, Error> {
        if let Some(body) = self.extensions_mut().remove::<Body>() {
            Ok(Some(to_bytes(body).await?))
        } else {
            Ok(None)
        }
    }

    /// Retrieve the value of a query string parameter
    ///
    /// # Examples
    ///
    /// ```
    /// use reign::prelude::*;
    ///
    /// #[action]
    /// async fn foo(req: &mut Request) -> Result<impl Response, Error> {
    ///     if let Some(val) = req.query("foo") {
    ///         Ok(val.clone())
    ///     } else {
    ///         Ok("No value".into())
    ///     }
    /// }
    /// ```
    #[inline]
    pub fn query(&self, name: &str) -> Option<&String> {
        self.query.get(name)
    }

    /// Retrieve a required path parameter
    ///
    /// # Examples
    ///
    /// ```
    /// use reign::prelude::*;
    ///
    /// #[action]
    /// async fn foo(req: &mut Request) -> Result<impl Response, Error> {
    ///     Ok(req.param("foo")?)
    /// }
    /// ```
    pub fn param(&self, name: &str) -> Result<String, ParamError> {
        Ok(self
            .params
            .get(name)
            .ok_or_else(|| ParamError::RequiredParamNotFound(name.into()))?
            .clone())
    }

    /// Retrieve an optional path parameter
    ///
    /// # Examples
    ///
    /// ```
    /// use reign::prelude::*;
    ///
    /// #[action]
    /// async fn foo(req: &mut Request) -> Result<impl Response, Error> {
    ///     if let Some(val) = req.param_opt("foo")? {
    ///         Ok(val)
    ///     } else {
    ///         Ok("No param".into())
    ///     }
    /// }
    /// ```
    pub fn param_opt(&self, name: &str) -> Result<Option<String>, ParamError> {
        Ok(self.params.get(name).cloned())
    }

    /// Retrieve a required path parameter
    ///
    /// # Examples
    ///
    /// ```
    /// use reign::prelude::*;
    ///
    /// #[action]
    /// async fn foo(req: &mut Request) -> Result<impl Response, Error> {
    ///     Ok(req.param_glob("foo")?.join("/"))
    /// }
    /// ```
    pub fn param_glob(&self, name: &str) -> Result<Vec<String>, ParamError> {
        Ok(self
            .params
            .get(name)
            .ok_or_else(|| ParamError::RequiredGlobParamNotFound(name.into()))?
            .clone()
            .split('/')
            .map(|x| x.into())
            .collect())
    }

    /// Retrieve an optional glob path parameter
    ///
    /// # Examples
    ///
    /// ```
    /// use reign::prelude::*;
    ///
    /// #[action]
    /// async fn foo(req: &mut Request) -> Result<impl Response, Error> {
    ///     if let Some(val) = req.param_opt_glob("foo")? {
    ///         Ok(val.join("/"))
    ///     } else {
    ///         Ok("No glob".into())
    ///     }
    /// }
    /// ```
    pub fn param_opt_glob(&self, name: &str) -> Result<Option<Vec<String>>, ParamError> {
        Ok(self
            .params
            .get(name)
            .cloned()
            .map(|x| x.split('/').map(|x| x.into()).collect()))
    }

    /// Retrieve the session data for the current session
    ///
    /// # Examples
    ///
    /// ```
    /// use reign::prelude::*;
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Serialize, Deserialize, Clone)]
    /// struct User(String);
    ///
    /// #[action]
    /// async fn foo(req: &mut Request) -> Result<impl Response, Error> {
    ///     if let Some(val) = req.session::<User>() {
    ///         Ok(val.clone().0)
    ///     } else {
    ///         Ok("No session".into())
    ///     }
    /// }
    /// ```
    #[cfg(feature = "session")]
    pub fn session<T>(&self) -> Option<&T>
    where
        T: Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static,
    {
        self.extensions()
            .get::<SessionData<T>>()
            .and_then(|data| match data {
                SessionData::Clean(data) => Some(data),
                _ => None,
            })
    }

    /// Store the session data for the current session
    ///
    /// # Examples
    ///
    /// ```
    /// use reign::prelude::*;
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Serialize, Deserialize)]
    /// struct User(String);
    ///
    /// #[action]
    /// async fn foo(req: &mut Request) -> Result<impl Response, Error> {
    ///     req.save_session(User("John".into()));
    ///     Ok("Saved session")
    /// }
    /// ```
    #[cfg(feature = "session")]
    pub fn save_session<T>(&mut self, data: T)
    where
        T: Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static,
    {
        self.extensions_mut().insert(SessionData::Dirty(data));
    }

    /// Delete the session data for the current session
    ///
    /// # Examples
    ///
    /// ```
    /// use reign::prelude::*;
    /// use serde::{Serialize, Deserialize};
    ///
    /// #[derive(Serialize, Deserialize)]
    /// struct User(String);
    ///
    /// #[action]
    /// async fn foo(req: &mut Request) -> Result<impl Response, Error> {
    ///     req.delete_session::<User>();
    ///     Ok("Deleted session")
    /// }
    /// ```
    #[cfg(feature = "session")]
    pub fn delete_session<T>(&mut self)
    where
        T: Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static,
    {
        if self.extensions().get::<SessionData<T>>().is_some() {
            self.extensions_mut().insert(SessionData::<T>::None);
        }
    }
}
