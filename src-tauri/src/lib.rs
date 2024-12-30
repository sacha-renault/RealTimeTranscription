mod database;

use database::db::{connect_to_db, DbConnection};
use database::{
    functions,
    model::{Chat, Message},
};
use tauri::{Manager, State};

#[tauri::command]
async fn get_chat_by_page(
    db: State<'_, DbConnection>,
    page_num: i64,
    page_size: i64,
) -> Result<Vec<Chat>, String> {
    // Retrieve the pool from managed state
    let pool = db.lock().await;

    // log call
    println!(
        "User requested page {} with page size {}",
        page_num, page_size
    );

    // Return the result
    Ok(functions::get_chats_by_page(&pool, page_num, page_size)
        .await
        .map_err(|e| e.to_string())?)
}

#[tauri::command]
async fn create_new_chat(
    db: State<'_, DbConnection>,
    title: String,
    description: Option<String>,
) -> Result<i64, String> {
    // log call
    println!("User created new chat : {}", title);

    // Retrieve the pool from managed state
    let pool = db.lock().await;

    // return id of created resource
    Ok(functions::create_new_chat(&pool, title, description)
        .await
        .map_err(|e| e.to_string())?)
}

#[tauri::command]
async fn get_chat_by_id(db: State<'_, DbConnection>, id: i64) -> Result<Chat, String> {
    // log call
    println!("User requested chat : {}", id);

    // Retrieve the pool from managed state
    let pool = db.lock().await;

    // return id of created resource
    Ok(functions::get_chat_by_id(&pool, id)
        .await
        .map_err(|e| e.to_string())?)
}

#[tauri::command]
async fn get_messages_by_chat_id(
    db: State<'_, DbConnection>,
    chat_id: i64,
) -> Result<Vec<Message>, String> {
    // log call
    println!("User requested messages of chat : {}", chat_id);

    // Retrieve the pool from managed state
    let pool = db.lock().await;

    // return id of created resource
    Ok(functions::get_messages_by_chat_id(&pool, chat_id)
        .await
        .map_err(|e| e.to_string())?)
}

#[tauri::command]
async fn add_new_message(
    db: State<'_, DbConnection>,
    chat_id: i64,
    content: String,
) -> Result<i64, String> {
    // log call
    println!("User created new message on chat: {}", chat_id);

    // Retrieve the pool from managed state
    let pool = db.lock().await;

    // return id of created resource
    Ok(functions::add_new_message(&pool, chat_id, content)
        .await
        .map_err(|e| e.to_string())?)
}

#[tauri::command]
async fn remove_chat_by_id(db: State<'_, DbConnection>, id: i64) -> Result<(), String> {
    // log call
    println!("User delete chat: {}", id);

    // Retrieve the pool from managed state
    let pool = db.lock().await;
    Ok(functions::remove_chat_by_id(&pool, id)
        .await
        .map_err(|e| e.to_string())?)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            let db = rt.block_on(connect_to_db(app.handle().clone()))?;
            app.manage(db);
            Ok(())
        })
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_chat_by_page,
            create_new_chat,
            get_chat_by_id,
            get_messages_by_chat_id,
            add_new_message,
            remove_chat_by_id
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
