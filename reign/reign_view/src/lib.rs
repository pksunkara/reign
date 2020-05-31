#![cfg_attr(feature = "doc", feature(external_doc))]
#![doc(html_root_url = "https://docs.rs/reign_view/0.1.2")]
#![cfg_attr(feature = "doc", doc(include = "../README.md"))]

#[doc(hidden)]
pub use maplit;

#[doc(hidden)]
pub mod parse;
mod slots;

#[doc(hidden)]
pub use slots::{slot_render, Slots};

#[cfg(feature = "view-router")]
use hyper::{header, http::Error, Body, Response, StatusCode};
#[cfg(feature = "view-router")]
use std::fmt::{write, Display};

/// Renders a view for [gotham](https://gotham.rs) handler.
///
/// The response is sent with content-type set as `text/html`.
///
/// *This function is available if Reign is built with the `"views-gotham"` feature.*
///
/// # Examples
///
/// ```
/// use reign::view::render_gotham;
/// # use std::fmt::{Formatter, Result, Display};
/// use gotham::state::State;
/// use gotham::hyper::{Body, Response};
/// # use gotham::{
/// #   router::builder::{build_simple_router, DrawRoutes, DefineSingleRoute},
/// #   init_server
/// # };
/// # use std::time::Duration;
/// # use tokio::{runtime::Runtime, select, time::delay_for};
///
/// # struct CustomView<'a> {
/// #   msg: &'a str
/// # }
/// #
/// # impl Display for CustomView<'_> {
/// #   fn fmt(&self, f: &mut Formatter) -> Result {
/// #       write!(f, "<h1>{}</h1>", self.msg)
/// #   }
/// # }
/// #
/// pub fn handler(state: State) -> (State, Response<Body>) {
///     (state, render_gotham(CustomView {
///         msg: "Hello Gotham!"
///     }, 200))
/// }
/// # let mut rt = Runtime::new().unwrap();
/// #
/// # rt.block_on(async {
/// #   let server = async {
/// #       let router = build_simple_router(|route| {
/// #           route.get("/").to(handler);
/// #       });
/// #
/// #       init_server("127.0.0.1:8080", router).await.unwrap()
/// #   };
/// #
/// #   let client = async {
/// #       delay_for(Duration::from_millis(100)).await;
/// #       let response = reqwest::get("http://localhost:8080").await.unwrap();
/// #
/// #       assert_eq!(response.status(), reqwest::StatusCode::OK);
/// #       assert!(response.headers().contains_key("content-type"));
/// #       assert_eq!(
/// #           response.headers()["content-type"],
/// #           "text/html; charset=utf-8"
/// #       );
/// #       assert_eq!(response.text().await.unwrap(), "<h1>Hello Gotham!</h1>");
/// #   };
/// #
/// #   select! {
/// #       _ = server => {}
/// #       _ = client => {}
/// #   }
/// # });
/// ```
#[cfg(feature = "view-router")]
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

/// Sends a redirect for [gotham](https://gotham.rs) handler.
///
/// The response is sent with status code `303` and `location` header.
///
/// *This function is available if Reign is built with the `"views-gotham"` feature.*
///
/// # Examples
///
/// ```
/// use reign::view::redirect_gotham;
/// use gotham::state::State;
/// use gotham::hyper::{Body, Response};
/// # use gotham::{
/// #   router::builder::{build_simple_router, DrawRoutes, DefineSingleRoute},
/// #   init_server
/// # };
/// # use std::time::Duration;
/// # use tokio::{runtime::Runtime, select, time::delay_for};
/// # use reqwest::{Client, redirect::Policy};
///
/// pub fn handler(state: State) -> (State, Response<Body>) {
///     (state, redirect_gotham("/dashboard"))
/// }
/// # let mut rt = Runtime::new().unwrap();
/// #
/// # rt.block_on(async {
/// #   let server = async {
/// #       let router = build_simple_router(|route| {
/// #           route.get("/").to(handler);
/// #       });
/// #
/// #       init_server("127.0.0.1:8080", router).await.unwrap()
/// #   };
/// #
/// #   let client = async {
/// #       delay_for(Duration::from_millis(100)).await;
/// #       let response = Client::builder()
/// #           .redirect(Policy::none())
/// #           .build()
/// #           .unwrap()
/// #           .get("http://localhost:8080")
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
#[cfg(feature = "view-router")]
pub fn redirect<L: AsRef<str>>(location: L) -> Result<Response<Body>, Error> {
    let mut response = Response::builder()
        .status(StatusCode::SEE_OTHER)
        .body(Body::empty())?;

    response
        .headers_mut()
        .insert(header::LOCATION, location.as_ref().parse().unwrap());

    Ok(response)
}

/// Serializes and sends JSON for [gotham](https://gotham.rs) handler.
///
/// The response is sent with content-type set as `application/json`.
///
/// *This function is available if Reign is built with the `"views-gotham"`
/// and `"json"` feature.*
///
/// # Examples
///
/// ```
/// use reign::view::json_gotham;
/// use gotham::state::State;
/// use gotham::hyper::{Body, Response};
/// # use gotham::{
/// #   router::builder::{build_simple_router, DrawRoutes, DefineSingleRoute},
/// #   init_server
/// # };
/// # use serde::Serialize;
/// # use std::time::Duration;
/// # use tokio::{runtime::Runtime, select, time::delay_for};
///
/// # #[derive(Serialize)]
/// # struct User<'a> {
/// #   name: &'a str
/// # }
/// #
/// pub fn handler(state: State) -> (State, Response<Body>) {
///     (state, json_gotham(User {
///         name: "Gotham"
///     }, 200))
/// }
/// # let mut rt = Runtime::new().unwrap();
/// #
/// # rt.block_on(async {
/// #   let server = async {
/// #       let router = build_simple_router(|route| {
/// #           route.get("/").to(handler);
/// #       });
/// #
/// #       init_server("127.0.0.1:8080", router).await.unwrap()
/// #   };
/// #
/// #   let client = async {
/// #       delay_for(Duration::from_millis(100)).await;
/// #       let response = reqwest::get("http://localhost:8080").await.unwrap();
/// #
/// #       assert_eq!(response.status(), reqwest::StatusCode::OK);
/// #       assert!(response.headers().contains_key("content-type"));
/// #       assert_eq!(
/// #           response.headers()["content-type"],
/// #           "application/json"
/// #       );
/// #       assert_eq!(response.text().await.unwrap(), r#"{"name":"Gotham"}"#);
/// #   };
/// #
/// #   select! {
/// #       _ = server => {}
/// #       _ = client => {}
/// #   }
/// # });
/// ```
#[cfg(feature = "view-router")]
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
