Reign Router is a feature-rich and fast HTTP server based on [Hyper](https://hyper.rs/) for
web applications in [Rust][].

This library was designed with many features needed for a web router while still keeping
simplicity and efficiency in mind. The result is one of the fastest [Rust][] routers and
also the most feature rich of them all.

The routing API of this library is heavily inspired by [Rails](https://rubyonrails.org)
and [Phoenix](https://www.phoenixframework.org) while the middleware API is inspired by
[Laravel](https://laravel.com) and [Django](https://www.djangoproject.com).

Please refer to [Guides](https://reign.rs/guides) on several how-to scenairos.

Please refer to [API documentation](https://docs.rs/reign_router) for more details.

**NOTE**: Minimum supported Rust version is **1.48.0**.

# Table of contents

1. [Quickstart](#quickstart)
2. [Features](#features)
3. [Concepts](#concepts)

# Quickstart

1. Add [Reign][] to your code base with default features ommitted and
   `router` feature enabled

    ```toml
    [dependencies]
    reign = { version = "*", features = ["router"], default-features = false }
    ```

2. Write an endpoint handler

    ```rust
    use reign::prelude::*;

    async fn foo(req: &mut Request) -> Result<impl Response, Error> {
        let id = req.param::<u32>("id")?;
        Ok(format!("foo {}", id))
    }
    ```

3. Define your routing rules

    ```rust,ignore
    use reign::router::Router;

    fn routes(r: &mut Router) {
        r.get(Path::new().path("foo").param("id"), foo);
        // can also be written as
        r.get(p!("foo" / id), foo);
    }
    ```

4. Serve the router

    ```rust,ignore
    use reign::router::serve;

    #[tokio::main]
    async fn main() {
        serve("127.0.0.1:8080", routes);
    }
    ```

# Features

### Path parameters

The router supports path parameters. Once defined in the router, the endpoint handler can request
any of these parameters from the request. The kinds of path parameters supported are:

* Required parameters
* Optional parameters
* Regex required parameters
* Regex optional parameters
* Glob required parameters
* Glob optional parameters

### Constraints

The router also supports constraints that allow the routes defined under them to either match
or not based on the request. The request has access to the following:

* URI (including host, port, path, query, etc..)
* HTTP Method
* Headers
* IP address
* Path parameters that are matched for this route

### Mutable State

The router redefines the incoming request object as a state and forwards it as a mutable pointer.
This means that the endpoint handler can not only interact more easily with the state but can also
use the **try** operator which makes the handler logic much more simplistic.

### Middlewares

The router comes with several middlewares which can be used in the router directly with minimal
configuration.

# Concepts

### Endpoint Handler

Each endpoint is defined by handler which takes incoming `Request` and responds with a `Result`
whose internal values implement the `Response` trait.

```rust
use reign::prelude::*;

async fn foo(req: &mut Request) -> Result<impl Response, Error> {
    Ok("foo")
}
```

The `&str` returned by the handler implements `Response` and the `Error` allowed here is
`reign::router::Error` which also implements it. You can use any `Error` here, all it needs
is to implement the `Response` trait.

### Middleware

All middlewares that can be used by the router needs to be implement `Middleware` trait which
involves implementing just one function. This function wraps the `Request` and `Response` allowing
the middleware to run both before and after the endpoint handler. Below shown is a sample logging
middleware.

```rust
use reign::router::{Chain, HandleFuture, Middleware, Request, futures::FutureExt};

pub struct Logger {}

impl Middleware for Logger {
    fn handle<'m>(&'m self, req: &'m mut Request, chain: Chain<'m>) -> HandleFuture<'m> {
        async move {
            println!("Request: from {} on {}", req.ip(), req.uri().path());
            let response = chain.run(req).await?;
            println!("Response: status {}", response.status());

            Ok(response)
        }.boxed()
    }
}
```

As a new request comes in, the router sends it to the first middleware which performs some logic
and in turn sends the request to the next middleware in the chain by calling `Chain::run`. The
final middleware forwards the request to the endpoint handler when it calls `Chain::run` which
produces a `hyper::Response`. Now, the middlewares will start executing post handler logic in the
reverse order.

Any middleware at any point in the whole chain can decide not to run it and just return a
`hyper::Response` thus aborting early.

[Rust]: https://rust-lang.org
[Reign]: https://docs.rs/reign
