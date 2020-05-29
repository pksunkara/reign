use hyper::{HeaderMap, Method, Uri, Version};

#[derive(Debug, Clone)]
pub struct Request {
    headers: HeaderMap,
    method: Method,
    uri: Uri,
    version: Version,
}

impl Request {
    pub fn method(&self) -> &Method {
        &self.method
    }

    pub fn version(&self) -> &Version {
        &self.version
    }
}
