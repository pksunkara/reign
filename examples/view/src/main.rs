use reign::prelude::*;

views!("src", "views");

fn handler() -> String {
    let page = "Home".to_string();
    let content = "Lorem ipsum";
    let count: u8 = 8;

    render!(app)
}

fn main() {
    println!("{}", handler());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handler() {
        assert_eq!(
            handler(),
            "<html>\n  <head>\n    <title>\n      App - Home\n    </title>\n  </head>\n  \
            <body>\n    \n  \n  <p>Lorem ipsum</p>\n  <span>8</span>\n  <i></i>\n\n  </body>\n</html>",
        );
    }
}
