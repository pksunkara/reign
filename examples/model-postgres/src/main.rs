use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use reign::{model::Database, prelude::*};

mod schema;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

#[derive(Debug, Model)]
struct User {
    id: i32,
    name: String,
}

async fn all_johns() -> Vec<User> {
    User::filter().name("John").all().await.unwrap()
}

fn connect() {
    Database::new("postgres://postgres@localhost:5432/reign_examples_model_postgres").connect();
    println!("Connected to the database");

    Database::get()
        .get()
        .unwrap()
        .run_pending_migrations(MIGRATIONS)
        .unwrap();
    println!("Ran database migrations");
}

#[tokio::main]
async fn main() {
    connect();

    let all_johns = all_johns().await;
    println!("All users named `John`: {:#?}", all_johns);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_all() {
        connect();
        let all_johns = all_johns().await;

        assert_eq!(all_johns.len(), 1);
        assert_eq!(all_johns[0].id, 1);
        assert_eq!(all_johns[0].name, "John");
    }
}
