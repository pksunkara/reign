#![cfg_attr(feature = "build-docs", feature(external_doc))]
#![doc(html_root_url = "https://docs.rs/reign_view/0.1.2")]
#![cfg_attr(feature = "build-docs", doc(include = "../README.md"))]

#[cfg(any(
    feature = "views-gotham",
    feature = "views-warp",
    feature = "views-tide",
    feature = "views-actix"
))]
use std::fmt::{write, Display};

#[doc(hidden)]
pub use maplit;

#[doc(hidden)]
pub mod parse;
mod slots;

#[doc(hidden)]
pub use slots::{slot_render, Slots};

/// Renders a view for [actix](https://actix.rs) request handler.
///
/// The response is sent with status code `200`
/// and content-type set as `text/html`.
///
/// *This function is available if Reign is built with the `"views-actix"` feature.*
///
/// # Examples
///
/// ```
/// use reign::view::render_actix;
/// # use std::fmt::{Formatter, Result, Display};
/// use actix_web::Responder;
/// # use actix_web::{web, App, HttpServer};
/// # use actix_rt::{System, spawn, time::delay_for};
/// # use std::time::Duration;
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
/// pub async fn handler() -> impl Responder {
///     render_actix(CustomView {
///         msg: "Hello Actix!"
///     })
/// }
/// #
/// # let mut rt = System::new("main");
/// #
/// # rt.block_on(async {
/// #   spawn(async {
/// #       HttpServer::new(|| App::new().route("/", web::get().to(handler)))
/// #           .bind("127.0.0.1:8080")
/// #           .unwrap()
/// #           .run()
/// #           .await
/// #           .unwrap();
/// #   });
/// #
/// #   delay_for(Duration::from_millis(100)).await;
/// #   let response = reqwest::get("http://localhost:8080").await.unwrap();
/// #
/// #   assert_eq!(response.status(), reqwest::StatusCode::OK);
/// #   assert!(response.headers().contains_key("content-type"));
/// #   assert_eq!(
/// #       response.headers()["content-type"],
/// #       "text/html; charset=utf-8"
/// #   );
/// #   assert_eq!(response.text().await.unwrap(), "<h1>Hello Actix!</h1>");
/// # });
/// ```
#[cfg(feature = "views-actix")]
pub fn render_actix<D: Display>(view: D) -> impl actix_web::Responder {
    use actix_web::{http::header::ContentType, HttpResponse};

    let mut content = String::new();

    match write(&mut content, format_args!("{}", view)) {
        Ok(()) => HttpResponse::Ok()
            .set(ContentType(mime::TEXT_HTML_UTF_8))
            .body(content),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/// Renders a view for [gotham](https://gotham.rs) handler.
///
/// The response is sent with status code `200`
/// and content-type set as `text/html`.
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
///     }))
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
#[cfg(feature = "views-gotham")]
pub fn render_gotham<D: Display>(view: D) -> gotham::hyper::Response<gotham::hyper::Body> {
    use gotham::hyper::{header, Body, Response, StatusCode};

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

/// Renders a view for [tide](https://docs.rs/tide) endpoint closure.
///
/// The response is sent with status code `200`
/// and content-type set as `text/html`.
///
/// *This function is available if Reign is built with the `"views-tide"` feature.*
///
/// # Examples
///
/// ```
/// use reign::view::render_tide;
/// # use std::fmt::{Formatter, Result, Display};
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
/// # let mut app = tide::new();
/// #
/// app.at("/").get(|_| async move {
///     render_tide(CustomView {
///         msg: "Hello Tide!"
///     })
/// });
/// #
/// # let mut rt = Runtime::new().unwrap();
/// #
/// # rt.block_on(async {
/// #   let server = async {
/// #       app.listen("127.0.0.1:8080").await.unwrap();
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
/// #       assert_eq!(response.text().await.unwrap(), "<h1>Hello Tide!</h1>");
/// #   };
/// #
/// #   select! {
/// #       _ = server => {}
/// #       _ = client => {}
/// #   }
/// # });
#[cfg(feature = "views-tide")]
pub fn render_tide<D: Display>(view: D) -> tide::Response {
    use tide::{http::StatusCode, Response};

    let mut content = String::new();

    match write(&mut content, format_args!("{}", view)) {
        Ok(()) => Response::new(StatusCode::OK.as_u16())
            .body_string(content)
            .set_mime(mime::TEXT_HTML_UTF_8),
        Err(_) => Response::new(StatusCode::INTERNAL_SERVER_ERROR.as_u16()),
    }
}

/// Renders a view for [warp](https://docs.rs/warp) closure.
///
/// The response is sent with status code `200`
/// and content-type set as `text/html`.
///
/// *This function is available if Reign is built with the `"views-warp"` feature.*
///
/// # Examples
///
/// ```
/// use reign::view::render_warp;
/// # use std::fmt::{Formatter, Result, Display};
/// use warp::Filter;
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
/// let app = warp::any().map(|| {
///     render_warp(CustomView {
///         msg: "Hello Warp!"
///     })
/// });
/// #
/// # let mut rt = Runtime::new().unwrap();
/// #
/// # rt.block_on(async {
/// #   let server = async move {
/// #       warp::serve(app).run(([127, 0, 0, 1], 8080)).await;
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
/// #       assert_eq!(response.text().await.unwrap(), "<h1>Hello Warp!</h1>");
/// #   };
/// #
/// #   select! {
/// #       _ = server => {}
/// #       _ = client => {}
/// #   }
/// # });
#[cfg(feature = "views-warp")]
pub fn render_warp<D: Display>(view: D) -> warp::hyper::Response<warp::hyper::Body> {
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

/// Sends a redirect for [actix](https://actix.rs) request handler.
///
/// The response is sent with status code `303` and `location` header.
///
/// *This function is available if Reign is built with the `"views-actix"` feature.*
///
/// # Examples
///
/// ```
/// use reign::view::redirect_actix;
/// use actix_web::Responder;
/// # use actix_web::{web, App, HttpServer};
/// # use actix_rt::{System, spawn, time::delay_for};
/// # use std::time::Duration;
/// # use reqwest::{Client, redirect::Policy};
///
/// pub async fn handler() -> impl Responder {
///     redirect_actix("/dashboard")
/// }
/// #
/// # let mut rt = System::new("main");
/// #
/// # rt.block_on(async {
/// #   spawn(async {
/// #       HttpServer::new(|| App::new().route("/", web::get().to(handler)))
/// #           .bind("127.0.0.1:8080")
/// #           .unwrap()
/// #           .run()
/// #           .await
/// #           .unwrap();
/// #   });
/// #
/// #   delay_for(Duration::from_millis(100)).await;
/// #   let response = Client::builder()
/// #       .redirect(Policy::none())
/// #       .build()
/// #       .unwrap()
/// #       .get("http://localhost:8080")
/// #       .send()
/// #       .await
/// #       .unwrap();
/// #
/// #   assert_eq!(response.status(), reqwest::StatusCode::SEE_OTHER);
/// #   assert!(response.headers().contains_key("location"));
/// #   assert_eq!(
/// #       response.headers()["location"],
/// #       "/dashboard"
/// #   );
/// #   assert_eq!(response.text().await.unwrap(), "");
/// # });
/// ```
#[cfg(feature = "views-actix")]
pub fn redirect_actix<L: AsRef<str>>(location: L) -> impl actix_web::Responder {
    use actix_web::{
        http::header::{HeaderValue, LOCATION},
        HttpResponse,
    };

    HttpResponse::SeeOther()
        .set_header(LOCATION, location.as_ref().parse::<HeaderValue>().unwrap())
        .finish()
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
#[cfg(feature = "views-gotham")]
pub fn redirect_gotham<L: AsRef<str>>(location: L) -> gotham::hyper::Response<gotham::hyper::Body> {
    use gotham::hyper::{header, Body, Response, StatusCode};

    let mut response = Response::builder()
        .status(StatusCode::SEE_OTHER)
        .body(Body::empty())
        .expect("Response built from a compatible type");

    response
        .headers_mut()
        .insert(header::LOCATION, location.as_ref().parse().unwrap());

    response
}

/// Sends a redirect for [tide](https://docs.rs/tide) endpoint closure.
///
/// The response is sent with status code `303` and `location` header.
///
/// *This function is available if Reign is built with the `"views-tide"` feature.*
///
/// # Examples
///
/// ```
/// use reign::view::redirect_tide;
/// # use std::time::Duration;
/// # use tokio::{runtime::Runtime, select, time::delay_for};
/// # use reqwest::{Client, redirect::Policy};
///
/// # let mut app = tide::new();
/// #
/// app.at("/").get(|_| async move {
///     redirect_tide("/dashboard")
/// });
/// #
/// # let mut rt = Runtime::new().unwrap();
/// #
/// # rt.block_on(async {
/// #   let server = async {
/// #       app.listen("127.0.0.1:8080").await.unwrap();
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
#[cfg(feature = "views-tide")]
pub fn redirect_tide<L: AsRef<str>>(location: L) -> tide::Response {
    use tide::{http::StatusCode, Response};

    Response::new(StatusCode::SEE_OTHER.as_u16()).set_header("location", location)
}

/// Sends a redirect for [warp](https://docs.rs/warp) closure.
///
/// The response is sent with status code `303` and `location` header.
///
/// *This function is available if Reign is built with the `"views-warp"` feature.*
///
/// # Examples
///
/// ```
/// use reign::view::redirect_warp;
/// use warp::Filter;
/// # use std::time::Duration;
/// # use tokio::{runtime::Runtime, select, time::delay_for};
/// # use reqwest::{Client, redirect::Policy};
///
/// let app = warp::any().map(|| {
///     redirect_warp("/dashboard")
/// });
/// #
/// # let mut rt = Runtime::new().unwrap();
/// #
/// # rt.block_on(async {
/// #   let server = async move {
/// #       warp::serve(app).run(([127, 0, 0, 1], 8080)).await;
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
#[cfg(feature = "views-warp")]
pub fn redirect_warp<L: AsRef<str>>(location: L) -> warp::hyper::Response<warp::hyper::Body> {
    use warp::hyper::{header, Body, Response, StatusCode};

    let mut response = Response::builder()
        .status(StatusCode::SEE_OTHER)
        .body(Body::empty())
        .expect("Response built from a compatible type");

    response
        .headers_mut()
        .insert(header::LOCATION, location.as_ref().parse().unwrap());

    response
}
