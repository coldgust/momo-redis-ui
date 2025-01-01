use crate::types::{ServerError, ServerResult};
use crate::AppState;
use api::{ClientCert, ConnAddr, ConnInfo};
use redis::aio::ConnectionManager;
use redis::{
    AsyncCommands, Client, ClientTlsConfig, ConnectionAddr, ConnectionInfo, ProtocolVersion,
    RedisConnectionInfo, TlsCertificates,
};
use std::fs;
use tauri::State;

#[tauri::command]
pub async fn set(state: State<'_, AppState>, key: String, val: String, info: ConnInfo) -> ServerResult<()> {
    let mut conn = get_or_create_connection(state, info).await?;
    conn.set(&key, val).await?;
    Ok(())
}

#[tauri::command]
pub async fn get(state: State<'_, AppState>, key: String, info: ConnInfo) -> ServerResult<String> {
    let mut conn = get_or_create_connection(state, info).await?;
    Ok(conn.get(&key).await?)
}

async fn get_or_create_connection(
    state: State<'_, AppState>,
    info: ConnInfo,
) -> ServerResult<ConnectionManager> {
    let key = &info.name.clone();
    {
        let map = state.connections_map.read().await;
        if map.contains_key(key) {
            return Ok(map.get(key).unwrap().clone());
        }
    }

    let mut map = state.connections_map.write().await;
    if map.contains_key(key) {
        return Ok(map.get(key).unwrap().clone());
    }

    let client = create_client(info).await?;
    let manager = ConnectionManager::new(client).await?;
    map.insert(key.to_string(), manager.clone());
    Ok(manager)
}

 async fn create_client(info: ConnInfo) -> ServerResult<Client> {
    let (host, port, db) = match info.addr {
        ConnAddr::Standalone(h, p, db) => (h, p, db),
        ConnAddr::Cluster(h, p) => (h, p, 0),
        _ => return Err(ServerError::UnsupportedConnType),
    };
    let redis_conn_info = RedisConnectionInfo {
        db,
        username: info.username,
        password: info.password,
        protocol: ProtocolVersion::RESP2,
    };
    let client = match info.ssl {
        Some(ref ssl) => Client::build_with_tls(
            ConnectionInfo {
                addr: ConnectionAddr::TcpTls {
                    host,
                    port,
                    insecure: ssl.hostname_verify,
                    tls_params: None,
                },
                redis: redis_conn_info,
            },
            TlsCertificates {
                client_tls: match &ssl.client_cert {
                    Some(ClientCert { cert, key }) => Some(ClientTlsConfig {
                        client_cert: fs::read(cert)?,
                        client_key: fs::read(key)?,
                    }),
                    None => None,
                },
                root_cert: match &ssl.ca_cert {
                    Some(path) => Some(fs::read(path)?),
                    None => None,
                },
            },
        ),
        None => Client::open(ConnectionInfo {
            addr: ConnectionAddr::Tcp(host, port),
            redis: redis_conn_info,
        }),
    }?;
    Ok(client)
}
