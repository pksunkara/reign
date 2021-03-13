use reign::prelude::*;

#[derive(Debug, Model)]
pub struct User {
    #[model(no_write)]
    pub id: i32,
    pub username: String,
    pub password: String,
    pub description: Option<String>,
    #[model(no_write)]
    pub created_at: chrono::NaiveDateTime,
}
