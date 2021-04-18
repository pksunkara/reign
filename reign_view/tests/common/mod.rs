use pretty_assertions;
use proc_macro2::TokenStream;
use reign_view::parse::{parse, tokenize};
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

macro_rules! eq {
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

pub fn parse_pass(file_name: &str) {
    let mut fixture = dir();
    let mut output = dir();

    fixture.push(&format!("{}.html", file_name));
    output.push(&format!("{}.rs", file_name));

    let f = read_to_string(fixture).unwrap().replace("\r\n", "\n");
    let o = read_to_string(output).unwrap();
    let t: TokenStream = o.parse().unwrap();

    let node = parse(f).unwrap();

    // TODO: test: Tokenstream should be converted to pretty formatted rust
    eq!(&t.to_string(), &tokenize(node).0.to_string());
    // eq!(&o.trim_end(), &tokenize(node).to_string());
}

pub fn parse_fail(file_name: &str) {
    let mut fixture = dir();
    let mut errlog = dir();

    fixture.push(&format!("{}.html", file_name));
    errlog.push(&format!("{}.err", file_name));

    let f = read_to_string(fixture).unwrap().replace("\r\n", "\n");
    let e = read_to_string(errlog).unwrap().replace("\r\n", "\n");

    eq!(&e, &format!("{:?}", parse(f).unwrap_err()));
}
