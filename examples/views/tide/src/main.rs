#![feature(proc_macro_hygiene)]
#![feature(type_ascription)]

use reign::prelude::*;
use tide;

views!("src", "views");

#[tokio::main]
async fn main() {
    let mut app = tide::new();

    app.at("/").get(|_| async move {
        let msg = "Hello World!";

        render!(views::App {
            _slots: reign::view::Slots::default(),
            msg
        })
    });

    app.listen("127.0.0.1:8080").await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server() {}
}
