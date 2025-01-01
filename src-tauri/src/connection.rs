use crate::types::ServerResult;
use crate::AppState;
use api::{ConnAddr, ConnInfo};
use sqlx::Executor;
use tauri::State;

#[tauri::command]
pub async fn create_connection(state: State<'_, AppState>, info: ConnInfo) -> ServerResult<()> {
    let (host, port, db_num, conn_type, node_pass, group_name) = match info.addr {
        ConnAddr::Standalone(h, p, d) => (h, p, d, "Standalone", None, None),
        ConnAddr::Cluster(h, p) => (h, p, 0, "Cluster", None, None),
        ConnAddr::Sentinel {
            host,
            port,
            node_password,
            master_group_name,
        } => (
            host,
            port,
            0,
            "Sentinel",
            node_password,
            Some(master_group_name),
        ),
    };

    let db = &state.db;
    sqlx::query("INSERT INTO connection(name, username, password, type, host, port, db, read_only, separator, \
    sentinel_node_password, sentinel_master_group_name, enable_ssl, cert_path, key_path, ca_path, hostname_verify) \
    VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)")
        .bind(&info.name)
        .bind(&info.username)
        .bind(&info.password)
        .bind(conn_type)
        .bind(host)
        .bind(port)
        .bind(db_num)
        .bind(info.read_only)
        .bind(info.separator)
        .bind(node_pass)
        .bind(group_name)
        .bind(info.ssl.is_some())
        .bind(info.ssl.as_ref().map(|s| s.client_cert.as_ref()).flatten().map(|c|&c.cert))
        .bind(info.ssl.as_ref().map(|s| s.client_cert.as_ref()).flatten().map(|c|&c.key))
        .bind(info.ssl.as_ref().map(|s| s.ca_cert.as_ref()).flatten())
        .bind(info.ssl.as_ref().map(|s| s.hostname_verify).unwrap_or(false))
        .execute(db)
        .await?;

    Ok(())
}
