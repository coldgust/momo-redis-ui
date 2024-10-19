use std::collections::HashMap;
use redis::Client;
use redis::cluster::ClusterClient;
use tauri::Manager;

mod redis_commands;

struct AppCache {
    client: HashMap<String, Client>,
    cluster_client: HashMap<String, ClusterClient>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(AppCache {
                client: HashMap::new(),
                cluster_client: HashMap::new(),
            });
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
