#![feature(proc_macro_hygiene)]

use reign::prelude::*;
use warp::Filter;

views!("src", "views");

#[tokio::main]
async fn main() {
    let app = warp::path::end().map(|| {
        let msg = "Hello World!";

        render!("app")
    });

    warp::serve(app).run(([127, 0, 0, 1], 8080)).await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server() {}
}
