use reign::prelude::*;

#[derive(Debug, Model)]
pub struct Article {
    #[model(no_insert, no_update)]
    pub id: i32,
    pub title: String,
    pub content: String,
}
