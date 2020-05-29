use anyhow::Error;
use futures::prelude::*;
use hyper::{
    http::Error as HttpError,
    server::{conn::AddrStream, Server},
    service::{make_service_fn, service_fn},
    Body, Error as HyperError, Method, Request as HyperRequest, Response as HyperResponse,
};
use std::{
    collections::HashMap as Map,
    convert::Infallible,
    net::{SocketAddr, ToSocketAddrs},
    sync::Arc,
};

mod middleware;
mod path;
mod pipe;
mod request;
mod response;
mod route;
mod scope;

pub use hyper;
pub use middleware::Middleware;
pub use path::Path;
pub use pipe::Pipe;
pub use request::Request;
pub use response::Response;
pub use route::Route;
pub use scope::Scope;

pub(crate) const INTERNAL_ERR: &'static str =
    "Internal error on reign_router. Please create an issue on https://github.com/pksunkara/reign";

macro_rules! method {
    ($name:ident, $method:ident) => {
        pub fn $name<H, R>(&mut self, path: Path<'a>, handler: H)
        where
            H: Fn(Request) -> R + Send + Sync + 'static,
            R: Future<Output = Result<HyperResponse<Body>, Error>> + Send + 'static,
        {
            self.any(&[Method::$method], path, handler);
        }
    };
}

#[derive(Default)]
pub struct Router<'a> {
    pipes: Map<&'a str, Pipe<'a>>,
    scopes: Vec<Scope<'a>>,
    routes: Vec<Route<'a>>,
}

impl<'a> Router<'a> {
    pub fn pipe(&mut self, pipe: Pipe<'a>) {
        self.pipes.insert(pipe.name, pipe);
    }

    pub fn scope(&mut self, scope: Scope<'a>) {
        self.scopes.push(scope);
    }

    pub fn any<H, R>(&mut self, methods: &[Method], path: Path<'a>, handler: H)
    where
        H: Fn(Request) -> R + Send + Sync + 'static,
        R: Future<Output = Result<HyperResponse<Body>, Error>> + Send + 'static,
    {
        self.routes
            .push(Route::new(path).methods(methods).handler(handler));
    }

    method!(get, GET);
    method!(post, POST);
    method!(put, PUT);
    method!(patch, PATCH);
    method!(delete, DELETE);
}

#[derive(Clone)]
pub(crate) struct RouterService<'a> {
    inner: Arc<Router<'a>>,
}

impl<'a> RouterService<'a> {
    pub(crate) fn new(router: Router<'a>) -> Self {
        Self {
            inner: Arc::new(router),
        }
    }

    pub(crate) async fn call_with_addr(
        self,
        req: HyperRequest<Body>,
        ip: SocketAddr,
    ) -> Result<HyperResponse<Body>, HttpError> {
        HyperResponse::builder().body(format!("{}", req.method().as_str()).into())
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
            "s".respond()
        }
        async fn multi_methods(req: Request) -> Result<HyperResponse<Body>, Error> {
            "s".respond()
        }
        async fn scope_static(req: Request) -> Result<HyperResponse<Body>, Error> {
            "s".respond()
        }

        fn router(r: &mut Router) {
            r.pipe(
                Pipe::new("common")
                    .and(HeadersDefault::empty().add("x-1", "a"))
                    .and(ContentType::empty().json()),
            );
            r.pipe(Pipe::new("timer").and(Runtime::default()));

            r.delete(Path::new("delete"), delete);

            r.any(
                &[Method::POST, Method::PUT],
                Path::new("multi_methods"),
                multi_methods,
            );

            r.scope(Scope::new(Path::new("scope_static")).to(|r| {
                r.get(Path::empty(), scope_static);
            }));
        }

        // serve("127.0.0.1:8080", router).await.unwrap();

        // let mut app = tide::new();

        // pipes.app(&["common"], &mut app);

        // app.at("/").get(|_| async move { Ok("hello") });
    }
}
