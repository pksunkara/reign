use hyper::{header, http::Error as HttpError, Body, Response as HyperResponse, StatusCode};

use std::fmt::{write, Display};

/// Renders a view for [reign router](reign_router) endpoint handle
///
/// The response is sent with content-type set as `text/html`.
///
/// # Examples
///
/// ```
/// use reign::prelude::*;
/// use std::fmt::{Display, Formatter, Result as FmtResult};
/// # use reign::router::serve;
/// # use std::time::Duration;
/// # use tokio::{runtime::Runtime, select, time::sleep};
///
/// struct CustomView<'a> {
///     msg: &'a str,
/// }
///
/// impl Display for CustomView<'_> {
///     fn fmt(&self, f: &mut Formatter) -> FmtResult {
///         write!(f, "<h1>{}</h1>", self.msg)
///     }
/// }
///
/// async fn handle(req: &mut Request) -> Result<impl Response, Error> {
///     Ok(render(
///         CustomView {
///             msg: "Hello Reign!",
///         },
///         200,
///     )?)
/// }
/// # let mut rt = Runtime::new().unwrap();
/// #
/// # rt.block_on(async {
/// #   let server = async {
/// #       serve("127.0.0.1:52525", |r| {
/// #           r.get("", handle);
/// #       }).await.unwrap()
/// #   };
/// #
/// #   let client = async {
/// #       sleep(Duration::from_millis(100)).await;
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
pub fn render<D: Display>(view: D, status: u16) -> Result<HyperResponse<Body>, HttpError> {
    let mut content = String::new();

    match write(&mut content, format_args!("{}", view)) {
        Ok(()) => {
            let status = StatusCode::from_u16(status)?;
            let mut response = HyperResponse::builder()
                .status(status)
                .body(Body::empty())?;

            response.headers_mut().insert(
                header::CONTENT_TYPE,
                mime::TEXT_HTML_UTF_8.as_ref().parse().unwrap(),
            );

            *response.body_mut() = content.into();

            Ok(response)
        }
        Err(_) => HyperResponse::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::empty()),
    }
}

/// Sends a redirect for [reign router](reign_router) endpoint
/// handle
///
/// The response is sent with status code `303` and `location` header.
///
/// # Examples
///
/// ```
/// use reign::prelude::*;
/// # use reign::router::serve;
/// # use std::time::Duration;
/// # use tokio::{runtime::Runtime, select, time::sleep};
/// # use reqwest::{Client, redirect::Policy};
///
/// async fn handle(req: &mut Request) -> Result<impl Response, Error> {
///     Ok(redirect("/dashboard")?)
/// }
/// # let mut rt = Runtime::new().unwrap();
/// #
/// # rt.block_on(async {
/// #   let server = async {
/// #       serve("127.0.0.1:52526", |r| {
/// #           r.get("", handle);
/// #       }).await.unwrap()
/// #   };
/// #
/// #   let client = async {
/// #       sleep(Duration::from_millis(100)).await;
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
pub fn redirect<L: AsRef<str>>(location: L) -> Result<HyperResponse<Body>, HttpError> {
    let mut response = HyperResponse::builder()
        .status(StatusCode::SEE_OTHER)
        .body(Body::empty())?;

    response
        .headers_mut()
        .insert(header::LOCATION, location.as_ref().parse().unwrap());

    Ok(response)
}

/// Serializes and sends JSON for [reign router](reign_router)
/// endpoint handle
///
/// The response is sent with content-type set as `application/json`.
///
/// # Examples
///
/// ```
/// use reign::prelude::*;
/// use serde::Serialize;
/// # use reign::router::serve;
/// # use std::time::Duration;
/// # use tokio::{runtime::Runtime, select, time::sleep};
///
/// #[derive(Serialize)]
/// struct User<'a> {
///     name: &'a str,
/// }
///
/// async fn handle(req: &mut Request) -> Result<impl Response, Error> {
///     Ok(json(User { name: "Reign" }, 200)?)
/// }
/// # let mut rt = Runtime::new().unwrap();
/// #
/// # rt.block_on(async {
/// #   let server = async {
/// #       serve("127.0.0.1:52527", |r| {
/// #           r.get("", handle);
/// #       }).await.unwrap()
/// #   };
/// #
/// #   let client = async {
/// #       sleep(Duration::from_millis(100)).await;
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
#[cfg(feature = "json")]
pub fn json<S: serde::Serialize>(value: S, status: u16) -> Result<HyperResponse<Body>, HttpError> {
    match serde_json::to_string::<S>(&value) {
        Ok(content) => {
            let status = StatusCode::from_u16(status)?;
            let mut response = HyperResponse::builder()
                .status(status)
                .body(Body::empty())?;

            response.headers_mut().insert(
                header::CONTENT_TYPE,
                mime::APPLICATION_JSON.as_ref().parse().unwrap(),
            );

            *response.body_mut() = content.into();

            Ok(response)
        }
        Err(_) => HyperResponse::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::empty()),
    }
}
