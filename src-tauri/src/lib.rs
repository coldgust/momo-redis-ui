use crate::redis_commands::{get, set};
use redis::aio::ConnectionManager;
use std::collections::HashMap;
use tauri::Manager;
use tokio::sync::RwLock;

mod redis_commands;
mod types;

#[derive(Default)]
pub struct AppState {
    connections_map: RwLock<HashMap<String, ConnectionManager>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            app.manage(AppState::default());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![set, get])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
