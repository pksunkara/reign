#[derive(Queryable)]
pub struct Article {
    pub id: i32,
    pub title: String,
    pub content: String,
}
