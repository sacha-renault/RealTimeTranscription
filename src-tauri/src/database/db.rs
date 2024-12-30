use sqlx::sqlite::{SqliteConnection, SqlitePool};
use sqlx::{Error, Executor};
use std::path::Path;
use std::sync::Arc;
use tauri::async_runtime::Mutex;
use tauri::{AppHandle, Manager};

pub type DbConnection = Arc<Mutex<SqlitePool>>;

async fn create_tables(pool: &SqlitePool) -> Result<(), Error> {
    let query = r#"
    CREATE TABLE IF NOT EXISTS chat (
        id INTEGER PRIMARY KEY,
        title TEXT NOT NULL,
        date DATETIME NOT NULL,
        description TEXT
    );

    CREATE TABLE IF NOT EXISTS message (
        id INTEGER PRIMARY KEY,
        chatId INTEGER NOT NULL,
        content TEXT NOT NULL,
        FOREIGN KEY (chatId) REFERENCES chat(id)
    );
    "#;

    // Execute the query to create the tables
    pool.execute(query).await?;

    Ok(())
}

pub async fn connect_to_db(app: AppHandle) -> Result<DbConnection, Error> {
    // Get the app's data directory
    let app_data_dir = app
        .path()
        .app_data_dir()
        .expect("Couldn't resolve the resource path.");

    // Ensure the directory exists
    std::fs::create_dir_all(&app_data_dir)?;

    // Path to the SQLite db
    let db_filename = "chat_db.db";

    // Define the database path inside the app's data folder
    let db_path = app_data_dir.join(db_filename);

    // Connect to the db
    // This will create a file if it doesn't exist
    let pool = SqlitePool::connect(&format!("sqlite://{}?mode=rwc", db_path.display())).await?;

    // Ensure the tables are created
    create_tables(&pool).await?;

    Ok(Arc::new(Mutex::new(pool)))
}
