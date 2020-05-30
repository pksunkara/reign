use futures::prelude::*;
use hyper::{
    server::{conn::AddrStream, Server},
    service::{make_service_fn, service_fn},
    Body, Error as HyperError, Method, Request as HyperRequest, Response as HyperResponse,
    StatusCode,
};
use log::debug;
use regex::{Regex, RegexSet};
use std::{
    collections::HashMap as Map,
    convert::Infallible,
    net::{SocketAddr, ToSocketAddrs},
    pin::Pin,
    sync::Arc,
};

mod error;
mod middleware;
mod path;
mod pipe;
mod request;
mod response;
mod route;
mod scope;

pub use hyper;

pub use error::*;
pub use middleware::Middleware;
pub use path::Path;
pub use pipe::Pipe;
pub use request::Request;
pub use response::Response;
use route::{Constraint, Handler, Route};
pub use scope::Scope;

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

#[derive(Debug, Clone)]
pub(crate) enum Index {
    Route(usize),
    Scope(usize),
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
        C: Fn(Request) -> bool + Send + Sync + 'static,
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
        C: Fn(Request) -> bool + Send + Sync + 'static,
        H: Fn(Request) -> R + Send + Sync + 'static,
        R: Future<Output = Result<HyperResponse<Body>, Error>> + Send + 'static,
    {
        self.routes
            .push(Route::new(path).constraint(constraint).handler(handler));
    }

    pub(crate) fn regex(&self) -> Vec<(Vec<Index>, (String, String))> {
        let mut routes = self
            .routes
            .iter()
            .enumerate()
            .map(|(i, x)| (vec![Index::Route(i)], x.regex()))
            .collect::<Vec<_>>();

        for (i, scope) in self.scopes.iter().enumerate() {
            let regex = scope.regex();

            for (j, route) in regex.1 {
                let mut index = vec![Index::Scope(i)];

                index.extend(j.into_iter());
                routes.push((index, (route.0, format!("{}{}", regex.0, route.1))));
            }
        }

        routes
    }

    pub(crate) fn get_handler(&self, index: &[Index]) -> Option<&Handler> {
        let first: &Index = index.get(0).expect(INTERNAL_ERR);

        match first {
            Index::Route(r) => self.routes.get(*r).expect(INTERNAL_ERR).handler.as_ref(),
            Index::Scope(s) => self
                .scopes
                .get(*s)
                .expect(INTERNAL_ERR)
                .router
                .get_handler(&index[1..]),
        }
    }

    pub(crate) fn build(&self) -> (Vec<String>, Vec<Vec<Index>>) {
        let regexes = self
            .regex()
            .into_iter()
            .map(|(i, x)| (i, format!("{}{}", x.0, x.1)))
            .collect::<Vec<_>>();

        (
            regexes.iter().map(|x| x.1.clone()).collect(),
            regexes.iter().map(|x| x.0.clone()).collect(),
        )
    }
}

#[derive(Clone)]
pub(crate) struct RouterService<'a> {
    router: Arc<Router<'a>>,
    regexes: Arc<Vec<Regex>>,
    indexes: Arc<Vec<Vec<Index>>>,
    regex_set: Arc<RegexSet>,
}

impl<'a> RouterService<'a> {
    pub(crate) fn new(router: Router<'a>) -> Self {
        let (regexes, indexes) = router.build();

        debug!("Route indexes: {:?}", indexes);
        debug!("Route regexes: {:?}", regexes);

        Self {
            router: Arc::new(router),
            regexes: Arc::new(
                regexes
                    .iter()
                    .map(|x| Regex::new(x).expect(INTERNAL_ERR))
                    .collect(),
            ),
            indexes: Arc::new(indexes),
            regex_set: Arc::new(RegexSet::new(regexes).expect(INTERNAL_ERR)),
        }
    }

    pub(crate) async fn call_with_addr(
        self,
        req: HyperRequest<Body>,
        ip: SocketAddr,
    ) -> Result<HyperResponse<Body>, Error> {
        let to_match = format!("{}{}", req.method().as_str(), req.uri().path());
        let to_match = to_match.trim_end_matches('/');
        let matches = self.regex_set.matches(to_match);

        if !matches.matched_any() {
            // TODO:(router:404) Check for 405 and support custom error handler through post middleware
            return Ok(HyperResponse::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())?);
        }

        let mut request = Request::new(ip, req);

        for m in matches {
            let regex = self.regexes.get(m).expect(INTERNAL_ERR);
            let index = self.indexes.get(m).expect(INTERNAL_ERR);

            debug!("Checking regex: {:?}", regex);

            let mut params = Map::new();
            let captures = regex.captures(to_match).expect(INTERNAL_ERR);

            for name in regex.capture_names() {
                if let Some(name) = name {
                    if let Some(value) = captures.name(name) {
                        params.insert(name.to_string(), value.as_str().to_string());
                    }
                }
            }

            debug!("Params extracted: {:?}", params);

            request.params = params;

            // TODO:(router) Check constaints

            if let Some(handler) = self.router.get_handler(index) {
                return Pin::from(handler(request)).await;
            }
        }

        Ok(HyperResponse::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::empty())?)
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

    let router_service = RouterService::new(router);

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
