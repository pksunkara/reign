use reign::prelude::*;

#[derive(Debug, Model)]
pub struct Article {
    #[model(no_write)]
    pub id: i32,
    pub title: String,
    pub content: String,
}
