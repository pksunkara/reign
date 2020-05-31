use reign::prelude::*;

fn test() {
    json!(200).unwrap();
    json!(200, status = 100).unwrap();
}

fn main() {}
