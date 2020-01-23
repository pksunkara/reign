#![feature(external_doc)]
#![doc(html_root_url = "https://docs.rs/reign_view/0.1.2")]
#![doc(include = "../README.md")]

use std::fmt::{self, write};

#[doc(hidden)]
pub use maplit;

pub mod parse;
mod slots;

pub use slots::{SlotRender, Slots};

#[cfg(feature = "views-gotham")]
use gotham::{
    helpers::http::response::{create_empty_response, create_response},
    state::State,
};

/// Renders a view for [gotham](https://gotham.rs) handler.
///
/// The response is sent with status code `200`
/// and content-type set as `text/html`.
///
/// *This function is available if the crate is built with the `"views-gotham"` feature.*
///
/// # Examples
///
/// ```
/// use reign::view::render_gotham;
/// # use std::fmt::{Formatter, Result, Display};
/// use gotham::state::State;
/// use gotham::hyper::{Body, Response};
/// # use gotham::test::TestServer;
/// # use gotham::hyper::StatusCode;
///
/// # struct CustomView<'a> {
/// #     msg: &'a str
/// # }
/// #
/// # impl Display for CustomView<'_> {
/// #     fn fmt(&self, f: &mut Formatter) -> Result {
/// #         write!(f, "<h1>{}</h1>", self.msg)
/// #     }
/// # }
/// #
/// pub fn handler(state: State) -> (State, Response<Body>) {
///     render_gotham(state, CustomView {
///         msg: "Hello World!"
///     })
/// }
/// #
/// # let test_server = TestServer::new(|| Ok(handler)).unwrap();
/// # let response = test_server
/// #     .client()
/// #     .get("http://localhost")
/// #     .perform()
/// #     .unwrap();
/// #
/// # assert_eq!(response.status(), StatusCode::OK);
/// # assert!(response.headers().contains_key("content-type"));
/// # assert_eq!(
/// #     response.headers()["content-type"],
/// #     "text/html; charset=utf-8"
/// # );
/// #
/// # let body = response.read_body().unwrap();
/// # assert_eq!(&body[..], b"<h1>Hello World!</h1>");
/// ```
#[cfg(feature = "views-gotham")]
pub fn render_gotham<D: fmt::Display>(
    state: State,
    view: D,
) -> (State, gotham::hyper::Response<gotham::hyper::Body>) {
    use gotham::hyper::StatusCode;

    let mut content = String::new();

    let response = match write(&mut content, format_args!("{}", view)) {
        Ok(()) => create_response(
            &state,
            StatusCode::OK,
            mime::TEXT_HTML_UTF_8,
            content.into_bytes(),
        ),
        Err(_) => create_empty_response(&state, StatusCode::INTERNAL_SERVER_ERROR),
    };

    (state, response)
}

/// Renders a view for [warp](https://docs.rs/warp) closure.
///
/// The response is sent with status code `200`
/// and content-type set as `text/html`.
///
/// *This function is available if the crate is built with the `"views-warp"` feature.*
///
/// # Examples
///
/// ```
/// use reign::view::render_warp;
/// # use std::fmt::{Formatter, Result, Display};
/// use warp::hyper::{Body, Response};
/// # use warp::hyper::StatusCode;
/// # use warp::Filter;
/// # use tokio::prelude::*;
/// # use tokio::runtime::Runtime;
/// #
/// # struct CustomView<'a> {
/// #     msg: &'a str
/// # }
/// #
/// # impl Display for CustomView<'_> {
/// #     fn fmt(&self, f: &mut Formatter) -> Result {
/// #         write!(f, "<h1>{}</h1>", self.msg)
/// #     }
/// # }
/// #
/// #
/// let filter = warp::any().map(|| {
///     render_warp(CustomView {
///         msg: "Hello World!"
///     })
/// });
/// #
/// # let mut rt = Runtime::new().unwrap();
/// #
/// # rt.block_on(async {
/// #     let response = warp::test::request()
/// #         .path("/")
/// #         .reply(&filter).await;
/// #
/// #     assert_eq!(response.status(), StatusCode::OK);
/// #     assert!(response.headers().contains_key("content-type"));
/// #     assert_eq!(
/// #         response.headers()["content-type"],
/// #         "text/html; charset=utf-8"
/// #     );
/// #
/// #     let body = response.body();
/// #     assert_eq!(&body[..], b"<h1>Hello World!</h1>");
/// # });
#[cfg(feature = "views-warp")]
pub fn render_warp<D: fmt::Display>(view: D) -> warp::hyper::Response<warp::hyper::Body> {
    use warp::hyper::{header, Body, Response, StatusCode};

    let mut content = String::new();

    match write(&mut content, format_args!("{}", view)) {
        Ok(()) => {
            let mut response = Response::builder()
                .status(StatusCode::OK)
                .body(Body::empty())
                .expect("Response built from a compatible type");

            response.headers_mut().insert(
                header::CONTENT_TYPE,
                mime::TEXT_HTML_UTF_8.as_ref().parse().unwrap(),
            );
            *response.body_mut() = content.into();
            response
        }
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::empty())
            .expect("Response built from a compatible type"),
    }
}
