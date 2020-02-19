#![feature(proc_macro_hygiene)]
#![feature(type_ascription)]

use gotham::{
    hyper::{Body, Response},
    router::{builder::*, Router},
    start,
    state::State,
};
use reign::prelude::*;

views!("src", "views");

fn hello(state: State) -> (State, Response<Body>) {
    let msg = "Hello World!";

    render!("app")
}

fn router() -> Router {
    build_simple_router(|route| {
        route.get("/").to(hello);
    })
}

fn main() {
    start("127.0.0.1:8080", router())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server() {}
}
