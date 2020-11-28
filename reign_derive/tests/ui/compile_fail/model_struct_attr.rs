use reign::prelude::*;

#[derive(Model)]
#[model(no_write)]
struct User {
    id: i32,
}

#[derive(Model)]
#[model(tag(id))]
struct Project {
    id: i32,
}

#[derive(Model)]
#[model(column_name = id)]
struct Team {
    id: i32,
}

fn main() {}
