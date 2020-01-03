use pretty_assertions;
use reign_view::parse::{parse, Node};
use std::env;
use std::fs::read_to_string;
use std::path::PathBuf;

#[derive(PartialEq, Eq)]
#[doc(hidden)]
pub struct PrettyString<'a>(pub &'a str);

/// Make diff to display string as multi-line string
impl<'a> std::fmt::Debug for PrettyString<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(self.0)
    }
}

macro_rules! assert_eq {
    ($left:expr, $right:expr) => {
        pretty_assertions::assert_eq!(PrettyString($left), PrettyString($right));
    };
}

fn dir() -> PathBuf {
    let mut dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    dir.push("tests");
    dir.push("fixtures");
    dir
}

pub fn parse_pass(file_name: &str) -> Node {
    let mut dir = dir();
    dir.push(&format!("{}.html", file_name));

    let contents = read_to_string(dir).unwrap();

    parse(contents).unwrap()
}

pub fn parse_fail(file_name: &str) {
    let mut fixture = dir();
    let mut errlog = dir();

    fixture.push(&format!("{}.html", file_name));
    errlog.push(&format!("{}.err", file_name));

    let f = read_to_string(fixture).unwrap();
    let e = read_to_string(errlog).unwrap();

    assert_eq!(&e, &format!("{:?}", parse(f).unwrap_err()));
}
