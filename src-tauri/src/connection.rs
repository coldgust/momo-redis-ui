use log::info;
use sqlx::Executor;
use tauri::State;
use api::{ConnAddr, ConnInfo};
use crate::AppState;
use crate::types::ServerResult;

#[tauri::command]
pub async fn create_connection(state: State<'_, AppState>, info: ConnInfo) -> ServerResult<()> {
    let db = &state.db;
    sqlx::query("INSERT INTO connection(name, username, password, type, host, port) \
    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)")
        .bind(&info.name)
        .bind(&info.username)
        .bind(&info.password)
        .bind(match &info.addr {
            ConnAddr::Standalone(_, _, _) => "Standalone",
            ConnAddr::Cluster(_, _, _) => "Cluster",
            ConnAddr::Sentinel { .. } => "Sentinel",
        })
        .execute(db)
        .await?;
    Ok(())
}