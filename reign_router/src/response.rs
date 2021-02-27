use hyper::{header, http::Error as HttpError, Body, Response as HyperResponse, StatusCode};
use mime::Mime;

use std::borrow::Cow;

/// Represents a type which can be converted into `hyper::Response`
///
/// # Examples
///
/// ```
/// use reign::{
///     prelude::*,
///     router::{
///         hyper::{http::Error as HttpError, Body, Response as HyperResponse, StatusCode}
///     },
/// };
///
/// struct NoContent;
///
/// impl Response for NoContent {
///     fn respond(self) -> Result<HyperResponse<Body>, HttpError> {
///         HyperResponse::builder()
///             .status(StatusCode::NO_CONTENT)
///             .body(Body::empty())
///     }
/// }
///
/// #[action]
/// async fn foo(req: &mut Request) -> Result<impl Response, Error> {
///     Ok(NoContent)
/// }
/// ```
pub trait Response {
    fn respond(self) -> Result<HyperResponse<Body>, HttpError>;
}

impl Response for HyperResponse<Body> {
    fn respond(self) -> Result<HyperResponse<Body>, HttpError> {
        Ok(self)
    }
}

impl<B> Response for (Mime, B)
where
    B: Into<Body>,
{
    fn respond(self) -> Result<HyperResponse<Body>, HttpError> {
        (StatusCode::OK, self.0, self.1).respond()
    }
}

impl<B> Response for (StatusCode, Mime, B)
where
    B: Into<Body>,
{
    fn respond(self) -> Result<HyperResponse<Body>, HttpError> {
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
    fn respond(self) -> Result<HyperResponse<Body>, HttpError> {
        HyperResponse::builder()
            .status(StatusCode::from_u16(self.0)?)
            .header(header::CONTENT_TYPE, self.1.as_ref())
            .body(self.2.into())
    }
}

macro_rules! plain_response {
    ($type:ty) => {
        impl Response for $type {
            fn respond(self) -> Result<HyperResponse<Body>, HttpError> {
                (StatusCode::OK, mime::TEXT_PLAIN, self).respond()
            }
        }
    };
}

plain_response!(&'static str);
plain_response!(Cow<'static, str>);
plain_response!(String);
