#![cfg_attr(feature = "doc", feature(external_doc))]
#![doc(html_logo_url = "https://reign.rs/images/media/reign.png")]
#![doc(html_root_url = "https://docs.rs/reign_view/0.2.1")]
#![cfg_attr(feature = "doc", doc(include = "../README.md"))]

#[doc(hidden)]
pub use maplit;

#[doc(hidden)]
pub mod parse;
mod slots;

#[doc(hidden)]
pub use slots::{slot_render, Slots};

#[cfg(feature = "view-backend")]
use hyper::{header, http::Error, Body, Response, StatusCode};
#[cfg(feature = "view-backend")]
use std::fmt::{write, Display};

/// Renders a view for [reign_router](https://docs.rs/reign_router) handler.
///
/// The response is sent with content-type set as `text/html`.
///
/// # Examples
///
/// ```
/// use reign::{
///     view::render,
///     router::{HandleFuture, Request, futures::FutureExt},
/// };
/// # use std::fmt::{Formatter, Result, Display};
/// # use reign::router::serve;
/// # use std::time::Duration;
/// # use tokio::{runtime::Runtime, select, time::delay_for};
///
/// struct CustomView<'a> {
///   msg: &'a str
/// }
///
/// impl Display for CustomView<'_> {
///   fn fmt(&self, f: &mut Formatter) -> Result {
///       write!(f, "<h1>{}</h1>", self.msg)
///   }
/// }
///
/// fn handler(req: &mut Request) -> HandleFuture {
///     async move {
///         Ok(render(CustomView {
///             msg: "Hello Reign!"
///         }, 200)?)
///     }.boxed()
/// }
/// # let mut rt = Runtime::new().unwrap();
/// #
/// # rt.block_on(async {
/// #   let server = async {
/// #       serve("127.0.0.1:52525", |r| {
/// #           r.get("", handler);
/// #       }).await.unwrap()
/// #   };
/// #
/// #   let client = async {
/// #       delay_for(Duration::from_millis(100)).await;
/// #       let response = reqwest::get("http://localhost:52525").await.unwrap();
/// #
/// #       assert_eq!(response.status(), reqwest::StatusCode::OK);
/// #       assert!(response.headers().contains_key("content-type"));
/// #       assert_eq!(
/// #           response.headers()["content-type"],
/// #           "text/html; charset=utf-8"
/// #       );
/// #       assert_eq!(response.text().await.unwrap(), "<h1>Hello Reign!</h1>");
/// #   };
/// #
/// #   select! {
/// #       _ = server => {}
/// #       _ = client => {}
/// #   }
/// # });
/// ```
#[cfg(feature = "view-backend")]
pub fn render<D: Display>(view: D, status: u16) -> Result<Response<Body>, Error> {
    let mut content = String::new();

    match write(&mut content, format_args!("{}", view)) {
        Ok(()) => match StatusCode::from_u16(status) {
            Ok(status) => {
                let mut response = Response::builder().status(status).body(Body::empty())?;

                response.headers_mut().insert(
                    header::CONTENT_TYPE,
                    mime::TEXT_HTML_UTF_8.as_ref().parse().unwrap(),
                );

                *response.body_mut() = content.into();

                Ok(response)
            }
            Err(_) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty()),
        },
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::empty()),
    }
}

/// Sends a redirect for [reign_router](https://docs.rs/reign_router) handler.
///
/// The response is sent with status code `303` and `location` header.
///
/// # Examples
///
/// ```
/// use reign::{
///     view::redirect,
///     router::{HandleFuture, Request, futures::FutureExt},
/// };
/// # use reign::router::serve;
/// # use std::time::Duration;
/// # use tokio::{runtime::Runtime, select, time::delay_for};
/// # use reqwest::{Client, redirect::Policy};
///
/// fn handler(req: &mut Request) -> HandleFuture {
///     async move {
///         Ok(redirect("/dashboard")?)
///     }.boxed()
/// }
/// # let mut rt = Runtime::new().unwrap();
/// #
/// # rt.block_on(async {
/// #   let server = async {
/// #       serve("127.0.0.1:52526", |r| {
/// #           r.get("", handler);
/// #       }).await.unwrap()
/// #   };
/// #
/// #   let client = async {
/// #       delay_for(Duration::from_millis(100)).await;
/// #       let response = Client::builder()
/// #           .redirect(Policy::none())
/// #           .build()
/// #           .unwrap()
/// #           .get("http://localhost:52526")
/// #           .send()
/// #           .await
/// #           .unwrap();
/// #
/// #       assert_eq!(response.status(), reqwest::StatusCode::SEE_OTHER);
/// #       assert!(response.headers().contains_key("location"));
/// #       assert_eq!(
/// #           response.headers()["location"],
/// #           "/dashboard"
/// #       );
/// #       assert_eq!(response.text().await.unwrap(), "");
/// #   };
/// #
/// #   select! {
/// #       _ = server => {}
/// #       _ = client => {}
/// #   }
/// # });
/// ```
#[cfg(feature = "view-backend")]
pub fn redirect<L: AsRef<str>>(location: L) -> Result<Response<Body>, Error> {
    let mut response = Response::builder()
        .status(StatusCode::SEE_OTHER)
        .body(Body::empty())?;

    response
        .headers_mut()
        .insert(header::LOCATION, location.as_ref().parse().unwrap());

    Ok(response)
}

/// Serializes and sends JSON for [reign_router](https://docs.rs/reign_router) handler.
///
/// The response is sent with content-type set as `application/json`.
///
/// # Examples
///
/// ```
/// use reign::{
///     view::json,
///     router::{HandleFuture, Request, futures::FutureExt},
/// };
/// # use reign::router::serve;
/// # use serde::Serialize;
/// # use std::time::Duration;
/// # use tokio::{runtime::Runtime, select, time::delay_for};
///
/// #[derive(Serialize)]
/// struct User<'a> {
///   name: &'a str
/// }
///
/// fn handler(req: &mut Request) -> HandleFuture {
///     async move {
///         Ok(json(User {
///             name: "Reign"
///         }, 200)?)
///     }.boxed()
/// }
/// # let mut rt = Runtime::new().unwrap();
/// #
/// # rt.block_on(async {
/// #   let server = async {
/// #       serve("127.0.0.1:52527", |r| {
/// #           r.get("", handler);
/// #       }).await.unwrap()
/// #   };
/// #
/// #   let client = async {
/// #       delay_for(Duration::from_millis(100)).await;
/// #       let response = reqwest::get("http://localhost:52527").await.unwrap();
/// #
/// #       assert_eq!(response.status(), reqwest::StatusCode::OK);
/// #       assert!(response.headers().contains_key("content-type"));
/// #       assert_eq!(
/// #           response.headers()["content-type"],
/// #           "application/json"
/// #       );
/// #       assert_eq!(response.text().await.unwrap(), r#"{"name":"Reign"}"#);
/// #   };
/// #
/// #   select! {
/// #       _ = server => {}
/// #       _ = client => {}
/// #   }
/// # });
/// ```
#[cfg(feature = "view-backend")]
pub fn json<S: serde::Serialize>(value: S, status: u16) -> Result<Response<Body>, Error> {
    match serde_json::to_string::<S>(&value) {
        Ok(content) => match StatusCode::from_u16(status) {
            Ok(status) => {
                let mut response = Response::builder().status(status).body(Body::empty())?;

                response.headers_mut().insert(
                    header::CONTENT_TYPE,
                    mime::APPLICATION_JSON.as_ref().parse().unwrap(),
                );

                *response.body_mut() = content.into();

                Ok(response)
            }
            Err(_) => Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty()),
        },
        Err(_) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::empty()),
    }
}
