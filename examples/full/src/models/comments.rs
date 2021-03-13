use reign::prelude::*;

#[derive(Debug, Model)]
pub struct Comment {
    #[model(no_write)]
    pub id: i32,
    pub text: String,
    pub user_id: i32,
    pub article_id: i32,
    #[model(no_write)]
    pub created_at: chrono::NaiveDateTime,
}
