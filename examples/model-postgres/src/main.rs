use diesel_migrations::embed_migrations;
use reign::{model::Database, prelude::*};

mod schema;

embed_migrations!("migrations");

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
}

#[tokio::main]
async fn main() {
    connect();
    println!("Connected to the database");

    embedded_migrations::run_with_output(&Database::get().get().unwrap(), &mut std::io::stdout())
        .unwrap();
    println!("Ran database migrations");

    let all_johns = all_johns().await;
    println!("All users named `John`: {:#?}", all_johns);
}

#[cfg(test)]
mod tests {
    use super::*;
    use reign::model::tokio_diesel::AsyncSimpleConnection;

    async fn setup() {
        connect();
        let conn = Database::get();

        conn.batch_execute_async("DROP TABLE IF EXISTS users")
            .await
            .unwrap();
        conn.batch_execute_async("CREATE TABLE users (id SERIAL, name VARCHAR(255))")
            .await
            .unwrap();
        conn.batch_execute_async("INSERT INTO users (name) VALUES ('John'), ('Sean')")
            .await
            .unwrap();
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_all() {
        setup().await;
        let all_johns = all_johns().await;

        assert_eq!(all_johns.len(), 1);
        assert_eq!(all_johns[0].id, 1);
        assert_eq!(all_johns[0].name, "John");
    }
}
