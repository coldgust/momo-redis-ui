use redis::{Client, ConnectionAddr, ConnectionInfo};
use tauri::State;
use tokio::sync::Mutex;
use api::{ConnAddr, ConnInfo};
use crate::AppCache;

async fn get_or_create_client<'a>(mut state: State<'a, Mutex<AppCache>>, info: &ConnInfo) -> Result<&'a Client, String> {
    let client_cache = &mut state.lock().await.client;
    if let Some(client) = client_cache.get(&info.name) {
        Ok(client)
    } else {
        let (host, port) = match &info.addr {
            ConnAddr::Standalone(h, p) => (h, p),
            _ => return Err("not support connection type".to_string()),
        };

        let conn_info = ConnectionInfo {
            addr: ConnectionAddr::TcpTls {
                host: host.to_string(),
                port: *port,
                insecure: info.ssl.as_ref().map_or(false, |params| params.hostname_verify),
                tls_params: None,
            },
            redis: Default::default(),
        };
        let client = Client::open(info.name.as_str()).unwrap();
    }
}