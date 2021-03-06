#[cfg(feature = "session")]
use crate::middleware::session::SessionData;
use crate::{
    hyper::{
        body::{to_bytes, Bytes},
        http::{request::Parts, Extensions},
        Body, HeaderMap, Method, Request as HyperRequest, Uri, Version,
    },
    Error, ParamError,
};

#[cfg(feature = "session")]
use serde::{Deserialize, Serialize};
use url::form_urlencoded::parse;

use std::{collections::HashMap as Map, net::SocketAddr, str::FromStr};

/// Request denotes the incoming request to the server and also acts as a state.
///
/// # Examples
///
/// ```
/// use reign::prelude::*;
///
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

    /// Returns a reference to the associated remote IP socket address.
    ///
    /// # Examples
    ///
    /// ```
    /// use reign::prelude::*;
    ///
    /// async fn foo(req: &mut Request) -> Result<impl Response, Error> {
    ///     Ok(req.ip().to_string())
    /// }
    /// ```
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
    /// async fn foo(req: &mut Request) -> Result<impl Response, Error> {
    ///     Ok((*req.method() == Method::GET).to_string())
    /// }
    /// ```
    #[inline]
    pub fn method(&self) -> &Method {
        &self.parts.method
    }

    /// Returns a reference to the associated version.
    ///
    /// # Examples
    ///
    /// ```
    /// use reign::{prelude::*, router::hyper::Version};
    ///
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

    /// Returns a reference to the underlying any-type storage.
    ///
    /// # Examples
    ///
    /// ```
    /// use reign::prelude::*;
    ///
    /// #[derive(Clone)]
    /// struct Custom(String);
    ///
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

    /// Returns a mutable reference to the underlying any-type storage.
    ///
    /// # Examples
    ///
    /// ```
    /// use reign::prelude::*;
    ///
    /// struct Custom(String);
    ///
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

    /// Retrieve the request body.
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

    /// Retrieve the value of a query string parameter.
    ///
    /// # Examples
    ///
    /// ```
    /// use reign::prelude::*;
    ///
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

    /// Retrieve the value of a required path parameter.
    ///
    /// # Examples
    ///
    /// ```
    /// use reign::prelude::*;
    ///
    /// async fn foo(req: &mut Request) -> Result<impl Response, Error> {
    ///     Ok(req.param::<String>("foo")?)
    /// }
    /// ```
    pub fn param<T>(&self, name: &str) -> Result<T, Error>
    where
        T: FromStr,
    {
        Ok(self
            .params
            .get(name)
            .ok_or_else(|| ParamError::RequiredParamNotFound(name.into()))
            .and_then(|p| {
                T::from_str(p).map_err(|_| ParamError::UnableToConvertParam(name.into()))
            })?)
    }

    /// Retrieve the value of an optional path parameter.
    ///
    /// # Examples
    ///
    /// ```
    /// use reign::prelude::*;
    ///
    /// async fn foo(req: &mut Request) -> Result<impl Response, Error> {
    ///     if let Some(val) = req.param_opt::<String>("foo")? {
    ///         Ok(val)
    ///     } else {
    ///         Ok("No param".into())
    ///     }
    /// }
    /// ```
    pub fn param_opt<T>(&self, name: &str) -> Result<Option<T>, Error>
    where
        T: FromStr,
    {
        Ok(self.params.get(name).map_or_else(
            || Ok(None),
            |p| {
                T::from_str(p)
                    .map_err(|_| ParamError::UnableToConvertParam(name.into()))
                    .map(Some)
            },
        )?)
    }

    /// Retrieve the value of a required glob path parameter.
    ///
    /// # Examples
    ///
    /// ```
    /// use reign::prelude::*;
    ///
    /// async fn foo(req: &mut Request) -> Result<impl Response, Error> {
    ///     Ok(req.param_glob::<String>("foo")?.join("/"))
    /// }
    /// ```
    pub fn param_glob<T>(&self, name: &str) -> Result<Vec<T>, Error>
    where
        T: FromStr,
    {
        Ok(self
            .params
            .get(name)
            .ok_or_else(|| ParamError::RequiredGlobParamNotFound(name.into()))
            .and_then(|p| {
                p.clone()
                    .split('/')
                    .map(|p| {
                        T::from_str(p).map_err(|_| ParamError::UnableToConvertParam(name.into()))
                    })
                    .collect::<Result<Vec<_>, _>>()
            })?)
    }

    /// Retrieve the value of an optional glob path parameter.
    ///
    /// # Examples
    ///
    /// ```
    /// use reign::prelude::*;
    ///
    /// async fn foo(req: &mut Request) -> Result<impl Response, Error> {
    ///     if let Some(val) = req.param_opt_glob::<String>("foo")? {
    ///         Ok(val.join("/"))
    ///     } else {
    ///         Ok("No glob".into())
    ///     }
    /// }
    /// ```
    pub fn param_opt_glob<T>(&self, name: &str) -> Result<Option<Vec<T>>, Error>
    where
        T: FromStr,
    {
        Ok(self.params.get(name).map_or_else(
            || Ok(None),
            |p| {
                p.clone()
                    .split('/')
                    .map(|p| {
                        T::from_str(p).map_err(|_| ParamError::UnableToConvertParam(name.into()))
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .map(Some)
            },
        )?)
    }

    /// Retrieve the session data for the current session.
    ///
    /// # Examples
    ///
    /// ```
    /// use reign::prelude::*;
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Serialize, Deserialize, Clone)]
    /// struct User(String);
    ///
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

    /// Store the session data for the current session.
    ///
    /// # Examples
    ///
    /// ```
    /// use reign::prelude::*;
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Serialize, Deserialize)]
    /// struct User(String);
    ///
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

    /// Delete the session data for the current session.
    ///
    /// # Examples
    ///
    /// ```
    /// use reign::prelude::*;
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Serialize, Deserialize)]
    /// struct User(String);
    ///
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

#[cfg(test)]
mod tests {
    use super::*;

    fn req_param(val: &str) -> Request {
        let mut req = Request::new(
            "10.10.10.10:80".parse().unwrap(),
            HyperRequest::get("https://reign.rs")
                .body(Body::empty())
                .unwrap(),
        );

        req.params.insert("id".into(), val.into());
        req
    }

    #[test]
    fn test_param() {
        let req = req_param("hey");
        let val = req.param::<String>("id");

        assert!(matches!(val, Ok(_)));
        assert_eq!(val.unwrap(), "hey");
    }

    #[test]
    fn test_param_not_found() {
        let req = req_param("hey");
        let val = req.param::<String>("none");

        assert!(matches!(
            val,
            Err(Error::Param(ParamError::RequiredParamNotFound(_)))
        ));
    }

    #[test]
    fn test_param_type() {
        let req = req_param("12");
        let val = req.param::<u32>("id");

        assert!(matches!(val, Ok(_)));
        assert_eq!(val.unwrap(), 12);
    }

    #[test]
    fn test_param_type_err() {
        let req = req_param("hey");
        let val = req.param::<u32>("id");

        assert!(matches!(
            val,
            Err(Error::Param(ParamError::UnableToConvertParam(_)))
        ));
    }

    #[test]
    fn test_param_opt() {
        let req = req_param("hey");
        let val = req.param_opt::<String>("id");

        assert!(matches!(val, Ok(Some(_))));
        assert_eq!(val.unwrap().unwrap(), "hey");
    }

    #[test]
    fn test_param_opt_not_found() {
        let req = req_param("hey");
        let val = req.param_opt::<String>("none");

        assert!(matches!(val, Ok(None)));
    }

    #[test]
    fn test_param_opt_type() {
        let req = req_param("12");
        let val = req.param_opt::<u32>("id");

        assert!(matches!(val, Ok(Some(_))));
        assert_eq!(val.unwrap().unwrap(), 12);
    }

    #[test]
    fn test_param_opt_type_err() {
        let req = req_param("hey");
        let val = req.param_opt::<u32>("id");

        assert!(matches!(
            val,
            Err(Error::Param(ParamError::UnableToConvertParam(_)))
        ));
    }

    #[test]
    fn test_param_glob() {
        let req = req_param("hey/wow");
        let val = req.param_glob::<String>("id");

        assert!(matches!(val, Ok(_)));

        let val = val.unwrap();

        assert_eq!(val.len(), 2);
        assert_eq!(val.get(0).unwrap(), "hey");
        assert_eq!(val.get(1).unwrap(), "wow");
    }

    #[test]
    fn test_param_glob_not_found() {
        let req = req_param("hey/wow");
        let val = req.param_glob::<String>("none");

        assert!(matches!(
            val,
            Err(Error::Param(ParamError::RequiredGlobParamNotFound(_)))
        ));
    }

    #[test]
    fn test_param_glob_type() {
        let req = req_param("12/34");
        let val = req.param_glob::<u32>("id");

        assert!(matches!(val, Ok(_)));

        let val = val.unwrap();

        assert_eq!(val.len(), 2);
        assert_eq!(val.get(0).unwrap(), &12);
        assert_eq!(val.get(1).unwrap(), &34);
    }

    #[test]
    fn test_param_glob_type_err() {
        let req = req_param("hey/wow");
        let val = req.param_glob::<u32>("id");

        assert!(matches!(
            val,
            Err(Error::Param(ParamError::UnableToConvertParam(_)))
        ));
    }

    #[test]
    fn test_param_opt_glob() {
        let req = req_param("hey/wow");
        let val = req.param_opt_glob::<String>("id");

        assert!(matches!(val, Ok(Some(_))));

        let val = val.unwrap().unwrap();

        assert_eq!(val.len(), 2);
        assert_eq!(val.get(0).unwrap(), "hey");
        assert_eq!(val.get(1).unwrap(), "wow");
    }

    #[test]
    fn test_param_opt_glob_not_found() {
        let req = req_param("hey/wow");
        let val = req.param_opt_glob::<String>("none");

        assert!(matches!(val, Ok(None)));
    }

    #[test]
    fn test_param_opt_glob_type() {
        let req = req_param("12/34");
        let val = req.param_opt_glob::<u32>("id");

        assert!(matches!(val, Ok(Some(_))));

        let val = val.unwrap().unwrap();

        assert_eq!(val.len(), 2);
        assert_eq!(val.get(0).unwrap(), &12);
        assert_eq!(val.get(1).unwrap(), &34);
    }

    #[test]
    fn test_param_opt_glob_type_err() {
        let req = req_param("hey/wow");
        let val = req.param_opt_glob::<u32>("id");

        assert!(matches!(
            val,
            Err(Error::Param(ParamError::UnableToConvertParam(_)))
        ));
    }
}
