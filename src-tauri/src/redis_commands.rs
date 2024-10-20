use api::error::Error;
use api::{ClientCert, ConnAddr, ConnInfo};
use redis::{AsyncCommands, Client, ClientTlsConfig, ConnectionAddr, ConnectionInfo, ProtocolVersion, RedisConnectionInfo, TlsCertificates};
use std::fs;

#[tauri::command]
pub async fn set(key: String, val: String, info: ConnInfo) -> Result<(), Error> {
    let client = create_client(info).await?;
    let mut conn = client.get_multiplexed_async_connection().await?;
    conn.set(&key, val).await?;
    Ok(())
}

#[tauri::command]
pub async fn get(key: String, info: ConnInfo) -> Result<String, Error> {
    let client = create_client(info).await?;
    let mut conn = client.get_multiplexed_async_connection().await?;
    Ok(conn.get(&key).await?)
}

async fn create_client(info: ConnInfo) -> Result<Client, Error> {
    let (host, port, db) = match info.addr {
        ConnAddr::Standalone(h, p, db) => (h, p, db),
        _ => return Err(Error::NonsupportConnType),
    };
    let redis_conn_info = RedisConnectionInfo {
        db,
        username: info.username,
        password: info.password,
        protocol: ProtocolVersion::RESP2,
    };
    let client = match info.ssl {
        Some(ref ssl) => {
            Client::build_with_tls(ConnectionInfo {
                addr: ConnectionAddr::TcpTls {
                    host,
                    port,
                    insecure: ssl.hostname_verify,
                    tls_params: None,
                },
                redis: redis_conn_info,
            }, TlsCertificates {
                client_tls: match &ssl.client_cert {
                    Some(ClientCert { cert, key }) => Some(ClientTlsConfig {
                        client_cert: fs::read(cert)?,
                        client_key: fs::read(key)?,
                    }),
                    None => None
                },
                root_cert: match &ssl.ca_cert {
                    Some(path) => Some(fs::read(path)?),
                    None => None,
                },
            })
        }
        None => {
            Client::open(ConnectionInfo {
                addr: ConnectionAddr::Tcp(host, port),
                redis: redis_conn_info,
            })
        }
    }?;
    Ok(client)
}