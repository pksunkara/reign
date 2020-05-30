use futures::prelude::*;
use hyper::{
    server::{conn::AddrStream, Server},
    service::{make_service_fn, service_fn},
    Body, Error as HyperError, Method, Response as HyperResponse,
};
use std::{collections::HashMap as Map, convert::Infallible, net::ToSocketAddrs};

mod error;
mod middleware;
mod path;
mod pipe;
mod request;
mod response;
mod route;
mod scope;
mod service;

pub use hyper;

pub use error::*;
pub use middleware::Middleware;
pub use path::Path;
pub use pipe::Pipe;
pub use request::Request;
pub use response::Response;
use route::{Constraint, Handler, Route};
pub use scope::Scope;
use service::{RouteRef, Service};

pub(crate) const INTERNAL_ERR: &'static str =
    "Internal error on reign_router. Please create an issue on https://github.com/pksunkara/reign";

macro_rules! method {
    ($name:ident, $method:ident) => {
        #[inline]
        pub fn $name<P, H, R>(&mut self, path: P, handler: H)
        where
            P: Into<Path<'a>>,
            H: Fn(Request) -> R + Send + Sync + 'static,
            R: Future<Output = Result<HyperResponse<Body>, Error>> + Send + 'static,
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

    pub fn scope<P, R>(&mut self, path: P, router_fn: R)
    where
        P: Into<Path<'a>>,
        R: Fn(&mut Router),
    {
        self.scopes.push(Scope::new(path).to(router_fn));
    }

    pub fn scope_through<P, R>(&mut self, path: P, pipes: &[&'a str], router_fn: R)
    where
        P: Into<Path<'a>>,
        R: Fn(&mut Router),
    {
        self.scopes
            .push(Scope::new(path).through(pipes).to(router_fn));
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
    pub fn any<P, H, R>(&mut self, methods: &[Method], path: P, handler: H)
    where
        P: Into<Path<'a>>,
        H: Fn(Request) -> R + Send + Sync + 'static,
        R: Future<Output = Result<HyperResponse<Body>, Error>> + Send + 'static,
    {
        self.routes
            .push(Route::new(path).methods(methods).handler(handler));
    }

    /// All methods allowed
    pub fn all<P, H, R>(&mut self, path: P, handler: H)
    where
        P: Into<Path<'a>>,
        H: Fn(Request) -> R + Send + Sync + 'static,
        R: Future<Output = Result<HyperResponse<Body>, Error>> + Send + 'static,
    {
        self.routes.push(Route::new(path).handler(handler));
    }

    /// Any of the given methods allowed with given constraint
    pub fn any_with_constraint<P, C, H, R>(
        &mut self,
        methods: &[Method],
        path: P,
        constraint: C,
        handler: H,
    ) where
        P: Into<Path<'a>>,
        C: Fn(&Request) -> bool + Send + Sync + 'static,
        H: Fn(Request) -> R + Send + Sync + 'static,
        R: Future<Output = Result<HyperResponse<Body>, Error>> + Send + 'static,
    {
        self.routes.push(
            Route::new(path)
                .methods(methods)
                .constraint(constraint)
                .handler(handler),
        );
    }

    /// All methods allowed with given constraint
    pub fn all_with_constraint<P, C, H, R>(&mut self, path: P, constraint: C, handler: H)
    where
        P: Into<Path<'a>>,
        C: Fn(&Request) -> bool + Send + Sync + 'static,
        H: Fn(Request) -> R + Send + Sync + 'static,
        R: Future<Output = Result<HyperResponse<Body>, Error>> + Send + 'static,
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
                constraints: vec![x.constraint.clone()],
            })
            .collect::<Vec<_>>();

        for scope in &self.scopes {
            let scope_ref = scope.refs();

            for route_ref in scope_ref.1 {
                let mut constraints = vec![scope_ref.0.clone()];

                constraints.extend(route_ref.constraints.into_iter());

                routes.push(RouteRef {
                    handler: route_ref.handler.clone(),
                    constraints: constraints.clone(),
                })
            }
        }

        routes
    }
}

pub async fn serve<A, R>(addr: A, router_fn: R) -> Result<(), HyperError>
where
    A: ToSocketAddrs + Send + 'static,
    R: Fn(&mut Router),
{
    let mut router = Router::default();
    router_fn(&mut router);

    let socket_addr = addr
        .to_socket_addrs()
        .expect("One of the socket address is not valid")
        .next()
        .expect("Must be given at least one socket address");

    let router_service = Service::new(router);

    let make_svc = make_service_fn(|socket: &AddrStream| {
        let remote_addr = socket.remote_addr();
        let router_service = router_service.clone();

        future::ok::<_, Infallible>(service_fn(move |req| {
            router_service.clone().call_with_addr(req, remote_addr)
        }))
    });

    Server::bind(&socket_addr).serve(make_svc).await
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::middleware::{ContentType, HeadersDefault, Runtime};
    // use tokio::test;

    #[test]
    fn test_server() {
        async fn delete(req: Request) -> Result<HyperResponse<Body>, Error> {
            Ok("delete".respond()?)
        }
        async fn multi_methods(req: Request) -> Result<HyperResponse<Body>, Error> {
            Ok("multi_methods".respond()?)
        }
        async fn scope_static(req: Request) -> Result<HyperResponse<Body>, Error> {
            Ok("scope_static".respond()?)
        }
        async fn constraint(req: Request) -> Result<HyperResponse<Body>, Error> {
            Ok("constraint".respond()?)
        }

        fn router(r: &mut Router) {
            r.pipe(
                Pipe::new("common")
                    .and(HeadersDefault::empty().add("x-1", "a"))
                    .and(ContentType::empty().json()),
            );
            r.pipe(Pipe::new("timer").and(Runtime::default()));

            r.delete("delete", delete);

            r.any(&[Method::POST, Method::PUT], "multi_methods", multi_methods);

            r.any_with_constraint(&[Method::GET], "constraint", |req| true, constraint);

            r.scope("scope_static", |r| {
                r.get(Path::new(), scope_static);
            });
        }

        // serve("127.0.0.1:8080", router).await.unwrap();
    }
}
