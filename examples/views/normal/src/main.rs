#![feature(proc_macro_hygiene)]

use reign::prelude::*;

views!("src", "views");

fn main() {
    let page = "Home";
    let content = "Lorem ipsum";
    let count: u8 = 8;

    println!("{}", render!("app"));
}

#[cfg(test)]
mod tests {
    use super::*;
}
