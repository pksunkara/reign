use reign::model::{diesel, tokio_diesel::AsyncSimpleConnection, Database};

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
        email -> Nullable<Varchar>,
    }
}

pub async fn setup() {
    let conn = Database::get_opt()
        .or_else(|| {
            Database::new("postgres://postgres@localhost:5432/reign_test").connect();
            Database::get_opt()
        })
        .unwrap();

    conn.batch_execute_async("DROP TABLE IF EXISTS users")
        .await
        .unwrap();
    conn.batch_execute_async(
        "CREATE TABLE users (id SERIAL, name VARCHAR(255) NOT NULL, email VARCHAR(255))",
    )
    .await
    .unwrap();
    conn.batch_execute_async("INSERT INTO users (name, email) VALUES ('John', NULL), ('Sean', 'sean@mail.com'), ('John', 'john@mail.com')")
        .await
        .unwrap();
}
