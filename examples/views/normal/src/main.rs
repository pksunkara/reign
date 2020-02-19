#![feature(proc_macro_hygiene)]
#![feature(type_ascription)]

use reign::prelude::*;

views!("src", "views");

fn main() {
    let page = "Home";
    let content = "Lorem ipsum";

    println!("{}", render!("app"));
}

#[cfg(test)]
mod tests {
    use super::*;
}
