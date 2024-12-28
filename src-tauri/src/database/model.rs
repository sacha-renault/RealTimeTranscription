use chrono::NaiveDateTime;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Chat {
    pub id: i64,
    pub title: String,
    pub date: NaiveDateTime,
    pub description: Option<String>,
}

#[derive(Debug, FromRow)]
pub struct Message {
    pub id: i64,
    pub chat_id: i64, // foreign key linking to Chat
    pub content: String,
}
