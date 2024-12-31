use super::model::{Chat, Message};
use chrono::{NaiveDateTime, Utc};
use sqlx::sqlite::SqlitePool;
use sqlx::{Error, FromRow};

pub async fn get_chats_by_page(
    pool: &SqlitePool,
    page: i64,
    page_size: i64,
) -> Result<Vec<Chat>, Error> {
    // Calculate the offset for pagination (page * page_size)
    let offset = page * page_size;

    // Query to fetch the chats, ordered by date (newest first)
    let query = r#"
    SELECT id, title, date, description
    FROM chat
    ORDER BY date DESC
    LIMIT ? OFFSET ?
    "#;

    // Fetch the results from the database
    let chats = sqlx::query_as::<_, Chat>(query)
        .bind(page_size)
        .bind(offset)
        .fetch_all(pool)
        .await?;

    Ok(chats)
}

// Create a new chat and return the id of the created chat
pub async fn create_new_chat(
    pool: &SqlitePool,
    title: String,
    description: Option<String>,
) -> Result<i64, Error> {
    // Use now as date
    let date = Utc::now().naive_utc();

    // insert new
    let result = sqlx::query(
        r"
        INSERT INTO chat (title, date, description)
        VALUES (?1, ?2, ?3)
        ",
    )
    .bind(title)
    .bind(date)
    .bind(description)
    .execute(pool)
    .await?;

    // Return the ID of the newly created chat
    Ok(result.last_insert_rowid())
}

// Get a chat by its id
pub async fn get_chat_by_id(pool: &SqlitePool, chat_id: i64) -> Result<Chat, Error> {
    // SQL query to fetch the chat by its ID
    let query = r#"
    SELECT id, title, date, description
    FROM chat
    WHERE id = ?1
    ORDER BY date DESC
    "#;

    // Fetch the chat from the database
    let chat = sqlx::query_as::<_, Chat>(query)
        .bind(chat_id)
        .fetch_one(pool)
        .await?;

    Ok(chat)
}

// get all the messages in a chat
pub async fn get_messages_by_chat_id(
    pool: &SqlitePool,
    chat_id: i64,
) -> Result<Vec<Message>, Error> {
    // SQL query to fetch the chat by its ID
    let query = r#"
    SELECT *
    FROM message
    WHERE chatId = ?1
    "#;

    // excecute query
    // Fetch the chat from the database
    let messages = sqlx::query_as::<_, Message>(query)
        .bind(chat_id)
        .fetch_all(pool)
        .await?;

    Ok(messages)
}

pub async fn add_new_message(
    pool: &SqlitePool,
    chat_id: i64,
    content: String,
) -> Result<i64, Error> {
    // insert new
    let result = sqlx::query(
        r"
        INSERT INTO message (chatId, content)
        VALUES (?1, ?2)
        ",
    )
    .bind(chat_id) // Bind the title
    .bind(content) // Bind the date
    .execute(pool)
    .await?;

    // Return the ID of the newly created chat
    Ok(result.last_insert_rowid())
}

pub async fn remove_chat_by_id(pool: &SqlitePool, id: i64) -> Result<(), Error> {
    let mut tx = pool.begin().await?; // Begin a transaction

    // Delete messages linked to the chat
    sqlx::query(
        r#"
            DELETE FROM message
            WHERE chatId = ?1
            "#,
    )
    .bind(id)
    .execute(&mut *tx)
    .await?;

    // Delete the chat
    sqlx::query(
        r#"
            DELETE FROM chat
            WHERE id = ?1
            "#,
    )
    .bind(id)
    .execute(&mut *tx)
    .await?;

    tx.commit().await?; // Commit the transaction

    Ok(())
}

pub async fn modify_chat_by_id(pool: &SqlitePool, id: i64, content: String) -> Result<(), Error> {
    // query to modify only content
    let result = sqlx::query(
        r"
        UPDATE message
        SET content = ?1
        WHERE id = ?2
        ",
    )
    .bind(content) // Bind the title
    .bind(id) // Bind the date
    .execute(pool)
    .await?;

    // Check if any row was actually updated
    if result.rows_affected() == 0 {
        return Err(Error::RowNotFound);
    }

    // Return the ID of the newly created chat
    Ok(())
}
