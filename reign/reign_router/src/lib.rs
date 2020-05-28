#![cfg_attr(feature = "doc", feature(external_doc))]
#![doc(html_root_url = "https://docs.rs/reign_router/0.1.2")]
#![cfg_attr(feature = "doc", doc(include = "../README.md"))]

use std::marker::PhantomData;
use std::{io::Result, net::ToSocketAddrs};

#[doc(hidden)]
pub mod builder;
pub mod middleware;
pub mod query;

pub enum Router<T = ()>
where
    T: Send + Sync + 'static,
{
    #[cfg(feature = "router-actix")]
    Actix(fn(&mut actix_web::web::ServiceConfig)),
    #[cfg(feature = "router-gotham")]
    Gotham(gotham::router::Router),
    #[cfg(feature = "router-tide")]
    Tide(tide::Server<T>),
    None(PhantomData<T>),
}

async fn serve_non_tide<A, T>(_addr: A, router: Router<T>) -> Result<()>
where
    A: ToSocketAddrs + Send + 'static,
    T: Send + Sync + 'static,
{
    match router {
        #[cfg(feature = "router-actix")]
        Router::Actix(c) => {
            actix_web::HttpServer::new(move || actix_web::App::new().configure(c))
                .bind(_addr)
                .unwrap()
                .run()
                .await
        }
        #[cfg(feature = "router-gotham")]
        Router::Gotham(r) => gotham::init_server(_addr, r)
            .await
            .map_err(|_| std::io::ErrorKind::Other.into()),
        _ => panic!("Can't recognize router"),
    }
}

#[cfg(feature = "router-tide")]
pub async fn serve<A, T>(_addr: A, router: Router<T>) -> Result<()>
where
    A: async_std::net::ToSocketAddrs + Send + 'static,
    T: Send + Sync + 'static,
{
    match router {
        #[cfg(feature = "router-tide")]
        Router::Tide(s) => s.listen(_addr).await,
        _ => {
            serve_non_tide(
                format!("{}", _addr.to_socket_addrs().await.unwrap().next().unwrap()),
                router,
            )
            .await
        }
    }
}

#[cfg(not(feature = "router-tide"))]
pub async fn serve<A, T>(_addr: A, router: Router<T>) -> Result<()>
where
    A: ToSocketAddrs + Send + 'static,
    T: Send + Sync + 'static,
{
    serve_non_tide(_addr, router).await
}
