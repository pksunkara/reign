use futures::prelude::*;
use hyper::{
    http::Error as HttpError,
    server::{conn::AddrStream, Server},
    service::{make_service_fn, service_fn},
    Body, Error as HyperError, Request, Response,
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
pub mod response;

pub use middleware::Middleware;
pub use path::Path;
pub use pipe::Pipe;

pub const INTERNAL_ERR: &'static str =
    "Internal error message. Please create an issue on https://github.com/pksunkara/reign";

pub struct Scope<'a> {
    name: &'a str,
}

#[derive(Default)]
pub struct Router<'a> {
    pipes: Map<&'a str, Pipe<'a>>,
}

impl<'a> Router<'a> {
    pub fn pipe(&mut self, pipe: Pipe<'a>) {
        self.pipes.insert(pipe.name, pipe);
    }
}

#[derive(Clone)]
pub(crate) struct RouterService<'a> {
    inner: Arc<Router<'a>>,
}

impl<'a> RouterService<'a> {
    pub fn new(router: Router<'a>) -> Self {
        Self {
            inner: Arc::new(router),
        }
    }

    pub async fn call_with_addr(
        self,
        req: Request<Body>,
        ip: SocketAddr,
    ) -> Result<Response<Body>, HttpError> {
        Response::builder().body(format!("{}", req.method().as_str()).into())
    }
}

pub async fn serve<A>(addr: A, router_fn: fn(&mut Router)) -> Result<(), HyperError>
where
    A: ToSocketAddrs + Send + 'static,
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
        fn router(r: &mut Router) {
            r.pipe(
                Pipe::new("common")
                    .and(HeadersDefault::empty().add("x-1", "a"))
                    .and(ContentType::empty().json()),
            );
            r.pipe(Pipe::new("timer").and(Runtime::default()));
        }

        // serve("127.0.0.1:8080", router).await.unwrap();

        // let mut app = tide::new();

        // pipes.app(&["common"], &mut app);

        // app.at("/").get(|_| async move { Ok("hello") });
    }
}
