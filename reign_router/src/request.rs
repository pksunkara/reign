use crate::{middleware::session::SessionData, ParamError};
use hyper::{
    body::{to_bytes, Bytes},
    http::{request::Parts, Extensions},
    Body, Error, HeaderMap, Method, Request as HyperRequest, Uri, Version,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap as Map, net::SocketAddr};
use url::form_urlencoded::parse;

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

    #[inline]
    pub fn ip(&self) -> &SocketAddr {
        &self.ip
    }

    /// Returns a reference to the associated Method.
    #[inline]
    pub fn method(&self) -> &Method {
        &self.parts.method
    }

    #[inline]
    pub fn version(&self) -> &Version {
        &self.parts.version
    }

    /// Returns a reference to the associated URI.
    #[inline]
    pub fn uri(&self) -> &Uri {
        &self.parts.uri
    }

    /// Returns a reference to the associated HeaderMap.
    #[inline]
    pub fn headers(&self) -> &HeaderMap {
        &self.parts.headers
    }

    #[inline]
    pub fn extensions(&self) -> &Extensions {
        &self.parts.extensions
    }

    #[inline]
    pub fn extensions_mut(&mut self) -> &mut Extensions {
        &mut self.parts.extensions
    }

    /// Retrieve the Request body.
    pub async fn body(&mut self) -> Result<Option<Bytes>, Error> {
        if let Some(body) = self.extensions_mut().remove::<Body>() {
            Some(to_bytes(body).await).transpose()
        } else {
            Ok(None)
        }
    }

    #[inline]
    pub fn query(&self, name: &str) -> Option<&String> {
        self.query.get(name)
    }

    pub fn param(&self, name: &str) -> Result<String, ParamError> {
        Ok(self
            .params
            .get(name)
            .ok_or_else(|| ParamError::RequiredParamNotFound(name.into()))?
            .clone())
    }

    pub fn param_opt(&self, name: &str) -> Result<Option<String>, ParamError> {
        Ok(self.params.get(name).cloned())
    }

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

    pub fn param_opt_glob(&self, name: &str) -> Result<Option<Vec<String>>, ParamError> {
        Ok(self
            .params
            .get(name)
            .cloned()
            .map(|x| x.split('/').map(|x| x.into()).collect()))
    }

    pub fn session<T>(&mut self) -> Option<&T>
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

    pub fn store_session<T>(&mut self, data: T)
    where
        T: Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static,
    {
        self.extensions_mut().insert(SessionData::Dirty(data));
    }

    pub fn drop_session<T>(&mut self)
    where
        T: Serialize + for<'de> Deserialize<'de> + Send + Sync + 'static,
    {
        if self.extensions().get::<SessionData<T>>().is_some() {
            self.extensions_mut().insert(SessionData::<T>::None);
        }
    }
}
