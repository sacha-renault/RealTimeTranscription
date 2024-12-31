use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct Chat {
    pub id: i64,
    pub title: String,
    pub date: NaiveDateTime,
    pub description: Option<String>,
}

#[derive(Debug, FromRow, Serialize)]
pub struct Message {
    pub id: i64,
    // #[sqlx(rename = "chatId")]
    // pub chat_id: i64,
    pub chatId: i64, // foreign key linking to Chat
    pub content: String,
}
