use std::env::var;
use std::path::PathBuf;
use walkdir::WalkDir;

fn main() {
    let mut dir = PathBuf::from(var("CARGO_MANIFEST_DIR").unwrap());

    dir.push("src");
    dir.push("views");

    for entry in WalkDir::new(dir) {
        println!("cargo:rerun-if-changed={}", entry.unwrap().path().display());
    }
}
