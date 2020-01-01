use reign_view::parse::{parse, Element};
use std::env;
use std::fs::read_to_string;
use std::path::PathBuf;

pub fn fixture(file_name: &str) -> Element {
    let mut dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    dir.push("tests");
    dir.push("fixtures");
    dir.push(&format!("{}.html", file_name));

    let contents = read_to_string(dir).unwrap();

    parse(contents)
}
