#![cfg_attr(feature = "doc", feature(external_doc))]
#![doc(html_logo_url = "https://reign.rs/images/media/reign.png")]
#![doc(html_root_url = "https://docs.rs/reign_router/0.2.1")]
#![cfg_attr(feature = "doc", doc(include = "../README.md"))]

use futures::future::ok;
use hyper::{
    server::{conn::AddrStream, Server},
    service::{make_service_fn, service_fn},
    Error as HyperError, Method,
};
use paste::paste;

use std::{collections::HashMap as Map, convert::Infallible, net::ToSocketAddrs};

pub use futures;
pub use hyper;

mod error;
mod path;
mod pipe;
mod request;
mod response;
mod route;
mod scope;
mod service;

pub mod middleware;

pub use error::*;
#[doc(inline)]
pub use middleware::{Chain, Middleware};
pub use path::Path;
pub use pipe::Pipe;
pub use request::Request;
pub use response::Response;
pub use route::HandleFuture;
pub use scope::Scope;
pub use service::{service, Service};

use pipe::MiddlewareItem;
use route::{Constraint, Handler, Route};
use service::RouteRef;

pub(crate) const INTERNAL_ERR: &str =
    "Internal error on reign_router. Please create an issue on https://github.com/pksunkara/reign";

macro_rules! method {
    ($method:ident) => {
        paste! {
            #[doc = " Define an endpoint with path that allows only `" $method:upper "` HTTP method"]
            ///
            /// # Examples
            ///
            /// ```
            /// use reign::router::Router;
            /// # use reign::prelude::*;
            /// #
            /// # #[action]
            /// # async fn foo(req: &mut Request) -> Result<impl Response, Error> { Ok("foo") }
            ///
            /// fn router(r: &mut Router) {
            #[doc = "     r." $method "(\"foo\", foo);"]
            /// }
            /// ```
            #[inline]
            pub fn $method<P, H>(&mut self, path: P, handler: H)
            where
                P: Into<Path>,
                H: Fn(&mut Request) -> HandleFuture + Send + Sync + 'static,
            {
                self.any(&[Method::[<$method:snake:upper>]], path, handler);
            }
        }
    };
}

/// Router that contains the routing rules and helpers to define them
///
/// # Examples
///
/// ```no_run
/// use reign::router::Router;
/// # use reign::{prelude::*, router::{Request, Response, Error}};
/// #
/// # #[action]
/// # async fn foo(req: &mut Request) -> Result<impl Response, Error> { Ok("foo") }
/// #
/// # #[action]
/// # async fn bar(req: &mut Request) -> Result<impl Response, Error> { Ok("bar") }
/// #
/// # #[action]
/// # async fn baz(req: &mut Request) -> Result<impl Response, Error> { Ok("baz") }
///
/// fn router(r: &mut Router) {
///     r.get("foo", foo);
///
///     r.scope("bar").to(|r| {
///         r.post("", bar);
///         r.delete("baz", baz);
///     });
/// }
/// ```
#[derive(Default)]
pub struct Router<'a> {
    pipes: Map<&'a str, Pipe>,
    scopes: Vec<Scope<'a>>,
    routes: Vec<Route>,
}

impl<'a> Router<'a> {
    /// Define a middleware pipe that can be used later
    ///
    /// # Examples
    ///
    /// ```
    /// use reign::router::{Router, middleware::Runtime};
    ///
    /// fn router(r: &mut Router) {
    ///     r.pipe("common").add(Runtime::default());
    /// }
    /// ```
    pub fn pipe(&mut self, name: &'a str) -> &mut Pipe {
        self.pipes.insert(name, Pipe::new());
        self.pipes.get_mut(name).expect(INTERNAL_ERR)
    }

    /// Define a scope with the given prefix
    ///
    /// # Examples
    ///
    /// ```
    /// use reign::router::Router;
    /// # use reign::prelude::*;
    /// #
    /// # #[action]
    /// # async fn foo(req: &mut Request) -> Result<impl Response, Error> { Ok("foo") }
    ///
    /// fn router(r: &mut Router) {
    ///     r.scope("api").to(|r| {
    ///         // GET /api/foo
    ///         r.get("foo", foo);
    ///     });
    /// }
    /// ```
    pub fn scope<P>(&mut self, path: P) -> &mut Scope<'a>
    where
        P: Into<Path>,
    {
        self.scopes.push(Scope::new(path));
        self.scopes.last_mut().expect(INTERNAL_ERR)
    }

    method!(get);
    method!(post);
    method!(put);
    method!(patch);
    method!(delete);
    method!(head);
    method!(options);
    method!(trace);
    method!(connect);

    /// Define an endpoint with path that allows any of the given HTTP methods
    ///
    /// # Examples
    ///
    /// ```
    /// use reign::router::{Router, hyper::Method};
    /// # use reign::prelude::*;
    /// #
    /// # #[action]
    /// # async fn foo(req: &mut Request) -> Result<impl Response, Error> { Ok("foo") }
    ///
    /// fn router(r: &mut Router) {
    ///     r.any(&[Method::GET], "foo", foo);
    /// }
    /// ```
    pub fn any<P, H>(&mut self, methods: &[Method], path: P, handler: H)
    where
        P: Into<Path>,
        H: Fn(&mut Request) -> HandleFuture + Send + Sync + 'static,
    {
        self.routes
            .push(Route::new(path).methods(methods).handler(handler));
    }

    /// Define an endpoint with path that allows all HTTP methods
    ///
    /// # Examples
    ///
    /// ```
    /// use reign::router::Router;
    /// # use reign::prelude::*;
    /// #
    /// # #[action]
    /// # async fn foo(req: &mut Request) -> Result<impl Response, Error> { Ok("foo") }
    ///
    /// fn router(r: &mut Router) {
    ///     r.all("foo", foo);
    /// }
    /// ```
    pub fn all<P, H>(&mut self, path: P, handler: H)
    where
        P: Into<Path>,
        H: Fn(&mut Request) -> HandleFuture + Send + Sync + 'static,
    {
        self.routes.push(Route::new(path).handler(handler));
    }

    /// Define an endpoint with path and constraint that allows any of the given HTTP methods.
    ///
    /// This endpoint will only be matched if the constraint returns true.
    ///
    /// # Examples
    ///
    /// ```
    /// use reign::router::{Router, hyper::Method};
    /// # use reign::prelude::*;
    /// #
    /// # #[action]
    /// # async fn foo(req: &mut Request) -> Result<impl Response, Error> { Ok("foo") }
    ///
    /// fn router(r: &mut Router) {
    ///     r.any_with_constraint(&[Method::GET], "foo", |req| {
    ///         req.uri().port().is_some() || req.query("bar").is_some()
    ///    }, foo);
    /// }
    /// ```
    pub fn any_with_constraint<P, C, H>(
        &mut self,
        methods: &[Method],
        path: P,
        constraint: C,
        handler: H,
    ) where
        P: Into<Path>,
        C: Fn(&Request) -> bool + Send + Sync + 'static,
        H: Fn(&mut Request) -> HandleFuture + Send + Sync + 'static,
    {
        self.routes.push(
            Route::new(path)
                .methods(methods)
                .constraint(constraint)
                .handler(handler),
        );
    }

    /// Define an endpoint with path and constraint that allows all HTTP methods.
    ///
    /// This endpoint will only be matched if the constraint returns true.
    ///
    /// # Examples
    ///
    /// ```
    /// use reign::router::Router;
    /// # use reign::prelude::*;
    /// #
    /// # #[action]
    /// # async fn foo(req: &mut Request) -> Result<impl Response, Error> { Ok("foo") }
    ///
    /// fn router(r: &mut Router) {
    ///     r.all_with_constraint("foo", |req| {
    ///         req.uri().port().is_some() || req.query("bar").is_some()
    ///    }, foo);
    /// }
    /// ```
    pub fn all_with_constraint<P, C, H>(&mut self, path: P, constraint: C, handler: H)
    where
        P: Into<Path>,
        C: Fn(&Request) -> bool + Send + Sync + 'static,
        H: Fn(&mut Request) -> HandleFuture + Send + Sync + 'static,
    {
        self.routes
            .push(Route::new(path).constraint(constraint).handler(handler));
    }
}

impl<'a> Router<'a> {
    pub(crate) fn regex(&self) -> Vec<(String, String)> {
        let mut regexes = self.routes.iter().map(|x| x.regex()).collect::<Vec<_>>();

        for scope in &self.scopes {
            let scope_regex = scope.regex();

            for route_regex in scope_regex.1 {
                regexes.push((route_regex.0, format!("{}{}", scope_regex.0, route_regex.1)))
            }
        }

        regexes
    }

    pub(crate) fn refs(&self) -> Vec<RouteRef> {
        let mut routes = self
            .routes
            .iter()
            .map(|x| RouteRef {
                handler: x.handler.clone(),
                middlewares: vec![],
                constraints: vec![x.constraint.clone()],
            })
            .collect::<Vec<_>>();

        for scope in &self.scopes {
            let scope_ref = scope.refs();

            for route_ref in scope_ref.1 {
                let mut constraints = vec![scope_ref.0.clone()];
                let mut middlewares = scope_ref
                    .2
                    .iter()
                    .flat_map(|x| {
                        let pipe = self.pipes.get(*x);

                        debug_assert!(pipe.is_some(), format!("can't find pipe with name `{}`", x));

                        pipe.map(|p| p.middlewares.clone()).unwrap_or(vec![])
                    })
                    .collect::<Vec<_>>();

                constraints.extend(route_ref.constraints.into_iter());
                middlewares.extend(route_ref.middlewares.into_iter());

                routes.push(RouteRef {
                    handler: route_ref.handler.clone(),
                    middlewares,
                    constraints,
                })
            }
        }

        routes
    }
}

/// Create the server using the given router definition
///
/// # Examples
///
/// ```no_run
/// use reign::router::{serve, Router};
///
/// fn router(r: &mut Router) {}
///
/// #[tokio::main]
/// async fn main() {
///     serve("127.0.0.1:8080", router).await.unwrap();
/// }
/// ```
pub async fn serve<A, R>(addr: A, f: R) -> Result<(), HyperError>
where
    A: ToSocketAddrs + Send + 'static,
    R: FnOnce(&mut Router),
{
    let router_service = service(f);

    let socket_addr = addr
        .to_socket_addrs()
        .expect("One of the socket address is not valid")
        .next()
        .expect("Must be given at least one socket address");

    let make_svc = make_service_fn(|socket: &AddrStream| {
        let remote_addr = socket.remote_addr();
        let router_service = router_service.clone();

        ok::<_, Infallible>(service_fn(move |req| {
            router_service.clone().call(req, remote_addr)
        }))
    });

    Server::bind(&socket_addr).serve(make_svc).await
}
