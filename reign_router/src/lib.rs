#![cfg_attr(feature = "doc", feature(external_doc))]
#![doc(html_logo_url = "https://reign.rs/images/media/reign.svg")]
#![doc(html_root_url = "https://docs.rs/reign_router/0.1.2")]
#![cfg_attr(feature = "doc", doc(include = "../README.md"))]

use futures::future::ok;
use hyper::{
    server::{conn::AddrStream, Server},
    service::{make_service_fn, service_fn},
    Error as HyperError, Method,
};
use std::{collections::HashMap as Map, convert::Infallible, net::ToSocketAddrs};

pub use anyhow;
pub use futures;
pub use hyper;
pub use log;
pub use tokio;

mod error;
mod path;
mod pipe;
mod request;
mod response;
mod route;
mod scope;
mod service;

#[cfg(feature = "file-handlers")]
pub mod handlers;
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

pub(crate) const INTERNAL_ERR: &'static str =
    "Internal error on reign_router. Please create an issue on https://github.com/pksunkara/reign";

macro_rules! method {
    ($name:ident, $method:ident) => {
        #[inline]
        pub fn $name<P, H>(&mut self, path: P, handler: H)
        where
            P: Into<Path<'a>>,
            H: Fn(&mut Request) -> HandleFuture + Send + Sync + 'static,
        {
            self.any(&[Method::$method], path, handler);
        }
    };
}

#[derive(Default)]
pub struct Router<'a> {
    in_scope: bool,
    pipes: Map<&'a str, Pipe<'a>>,
    scopes: Vec<Scope<'a>>,
    routes: Vec<Route<'a>>,
}

impl<'a> Router<'a> {
    pub(crate) fn in_scope() -> Self {
        Self {
            in_scope: true,
            ..Default::default()
        }
    }

    pub fn pipe(&mut self, pipe: Pipe<'a>) {
        if self.in_scope {
            panic!("Pipes are not allowed to be defined in scopes");
        }

        self.pipes.insert(pipe.name, pipe);
    }

    pub fn scope<P, R>(&mut self, path: P, f: R)
    where
        P: Into<Path<'a>>,
        R: Fn(&mut Router),
    {
        self.scopes.push(Scope::new(path).to(f));
    }

    pub fn scope_through<P, R>(&mut self, path: P, pipes: &[&'a str], f: R)
    where
        P: Into<Path<'a>>,
        R: Fn(&mut Router),
    {
        self.scopes.push(Scope::new(path).through(pipes).to(f));
    }

    pub fn scope_as(&mut self, scope: Scope<'a>) {
        self.scopes.push(scope);
    }

    method!(get, GET);
    method!(post, POST);
    method!(put, PUT);
    method!(patch, PATCH);
    method!(delete, DELETE);
    method!(head, HEAD);
    method!(options, OPTIONS);
    method!(trace, TRACE);
    method!(connect, CONNECT);

    /// Any of the given methods allowed
    pub fn any<P, H>(&mut self, methods: &[Method], path: P, handler: H)
    where
        P: Into<Path<'a>>,
        H: Fn(&mut Request) -> HandleFuture + Send + Sync + 'static,
    {
        self.routes
            .push(Route::new(path).methods(methods).handler(handler));
    }

    /// All methods allowed
    pub fn all<P, H>(&mut self, path: P, handler: H)
    where
        P: Into<Path<'a>>,
        H: Fn(&mut Request) -> HandleFuture + Send + Sync + 'static,
    {
        self.routes.push(Route::new(path).handler(handler));
    }

    /// Any of the given methods allowed with given constraint
    pub fn any_with_constraint<P, C, H>(
        &mut self,
        methods: &[Method],
        path: P,
        constraint: C,
        handler: H,
    ) where
        P: Into<Path<'a>>,
        C: Fn(&Request) -> bool + Send + Sync + 'static,
        H: Fn(&mut Request) -> HandleFuture + Send + Sync + 'static,
    {
        self.routes.push(
            Route::new(path)
                .methods(methods)
                .constraint(constraint)
                .handler(move |req| handler(req)),
        );
    }

    /// All methods allowed with given constraint
    pub fn all_with_constraint<P, C, H>(&mut self, path: P, constraint: C, handler: H)
    where
        P: Into<Path<'a>>,
        C: Fn(&Request) -> bool + Send + Sync + 'static,
        H: Fn(&mut Request) -> HandleFuture + Send + Sync + 'static,
    {
        self.routes
            .push(Route::new(path).constraint(constraint).handler(handler));
    }

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
                        if let Some(pipe) = self.pipes.get(*x) {
                            pipe.middlewares.clone()
                        } else {
                            vec![]
                        }
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

pub async fn serve<A, R>(addr: A, f: R) -> Result<(), HyperError>
where
    A: ToSocketAddrs + Send + 'static,
    R: Fn(&mut Router),
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
