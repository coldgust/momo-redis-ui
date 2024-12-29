use crate::migration::{setup_db, Db};
use crate::redis_commands::{get, set};
use redis::aio::ConnectionManager;
use std::collections::HashMap;
use tauri::Manager;
use tokio::sync::RwLock;

mod migration;
mod redis_commands;
mod types;

pub struct AppState {
    connections_map: RwLock<HashMap<String, ConnectionManager>>,
    db: Db,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            tauri::async_runtime::block_on(async move {
                let db = setup_db(&app).await;
                app.manage(AppState {
                    db,
                    connections_map: RwLock::new(HashMap::new()),
                })
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![set, get])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
