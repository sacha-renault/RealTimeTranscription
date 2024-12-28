use super::model::{Chat, Message};
use chrono::NaiveDateTime;
use sqlx::sqlite::SqlitePool;
use sqlx::{Error, FromRow};

// async fn get_chats_by_page(
//     pool: &SqlitePool,
//     page: i64,      // Page number, 0-based
//     page_size: i64, // Number of results per page
// ) -> Result<Vec<Chat>, Error> {
//     // Calculate the offset for pagination (page * page_size)
//     let offset = page * page_size;

//     // Query to fetch the chats, ordered by date (newest first)
//     let query = r#"
//     SELECT id, title, date, description
//     FROM chat
//     ORDER BY date DESC
//     LIMIT ? OFFSET ?
//     "#;

//     // Fetch the results from the database
//     let chats = sqlx::query_as::<_, Chat>(query)
//         .bind(page_size) // LIMIT
//         .bind(offset) // OFFSET
//         .fetch_all(pool)
//         .await?;

//     Ok(chats)
// }
