use reign::prelude::*;

#[derive(Model)]
struct User {
    #[model(table_name = users)]
    id: i32,
}

#[derive(Model)]
struct Org {
    #[model(primary_key(foo))]
    foo: i32,
}

fn main() {}
