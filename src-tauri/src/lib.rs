use crate::redis_commands::{get, set};
use tauri::Manager;

mod redis_commands;
mod types;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![set, get])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
