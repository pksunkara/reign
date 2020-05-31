use crate::router::{
    hyper::{http::Extensions, Body, HeaderMap, Method, Request as HyperRequest, Uri, Version},
    ParamError,
};
use std::{collections::HashMap as Map, net::SocketAddr};
use url::form_urlencoded::parse;

#[derive(Debug)]
pub struct Request {
    pub(crate) ip: SocketAddr,
    pub(crate) method: Method,
    pub(crate) version: Version,
    pub(crate) uri: Uri,
    pub(crate) headers: HeaderMap,
    pub(crate) params: Map<String, String>,
    pub(crate) query: Map<String, String>,
    pub extensions: Extensions,
}

impl Request {
    pub(crate) fn new(ip: SocketAddr, req: HyperRequest<Body>) -> Self {
        Self {
            ip,
            method: req.method().clone(),
            version: req.version(),
            uri: req.uri().clone(),
            headers: req.headers().clone(),
            params: Map::new(),
            query: req
                .uri()
                .query()
                .map(|v| parse(v.as_bytes()).into_owned().collect())
                .unwrap_or_else(Map::new),
            extensions: Extensions::new(),
        }
    }

    #[inline]
    pub fn ip(&self) -> &SocketAddr {
        &self.ip
    }

    #[inline]
    pub fn method(&self) -> &Method {
        &self.method
    }

    #[inline]
    pub fn version(&self) -> &Version {
        &self.version
    }

    #[inline]
    pub fn uri(&self) -> &Uri {
        &self.uri
    }

    #[inline]
    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }

    pub fn query(&self, name: &str) -> Option<&String> {
        self.query.get(name)
    }

    pub fn param(&self, name: &str) -> Result<String, ParamError> {
        Ok(self
            .params
            .get(name)
            .ok_or(ParamError::RequiredParamNotFound(name.into()))?
            .clone())
    }

    pub fn param_opt(&self, name: &str) -> Result<Option<String>, ParamError> {
        Ok(self.params.get(name).cloned())
    }

    pub fn param_glob(&self, name: &str) -> Result<Vec<String>, ParamError> {
        Ok(self
            .params
            .get(name)
            .ok_or(ParamError::RequiredGlobParamNotFound(name.into()))?
            .clone()
            .split("/")
            .into_iter()
            .map(|x| x.into())
            .collect())
    }

    pub fn param_opt_glob(&self, name: &str) -> Result<Option<Vec<String>>, ParamError> {
        Ok(self
            .params
            .get(name)
            .cloned()
            .map(|x| x.split("/").into_iter().map(|x| x.into()).collect()))
    }
}
