use hyper::{header, http::Error, Body, Response as HyperResponse, StatusCode};
use mime::Mime;
use std::borrow::Cow;

pub trait Response {
    fn respond(self) -> Result<HyperResponse<Body>, Error>;
}

impl Response for HyperResponse<Body> {
    fn respond(self) -> Result<HyperResponse<Body>, Error> {
        Ok(self)
    }
}

impl<B> Response for (Mime, B)
where
    B: Into<Body>,
{
    fn respond(self) -> Result<HyperResponse<Body>, Error> {
        (StatusCode::OK, self.0, self.1).respond()
    }
}

impl<B> Response for (StatusCode, Mime, B)
where
    B: Into<Body>,
{
    fn respond(self) -> Result<HyperResponse<Body>, Error> {
        HyperResponse::builder()
            .status(self.0)
            .header(header::CONTENT_TYPE, self.1.as_ref())
            .body(self.2.into())
    }
}

impl<B> Response for (u16, Mime, B)
where
    B: Into<Body>,
{
    fn respond(self) -> Result<HyperResponse<Body>, Error> {
        HyperResponse::builder()
            .status(StatusCode::from_u16(self.0)?)
            .header(header::CONTENT_TYPE, self.1.as_ref())
            .body(self.2.into())
    }
}

macro_rules! plain_response {
    ($type:ty) => {
        impl Response for $type {
            fn respond(self) -> Result<HyperResponse<Body>, Error> {
                (StatusCode::OK, mime::TEXT_PLAIN, self).respond()
            }
        }
    };
}

plain_response!(&'static str);
plain_response!(Cow<'static, str>);
plain_response!(String);
