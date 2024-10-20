pub mod error;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConnInfo {
    pub name: String,
    pub addr: ConnAddr,
    pub read_only: bool,
    pub username: Option<String>,
    pub password: Option<String>,
    pub separator: Option<String>,
    pub ssl: Option<SslParams>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ConnAddr {
    Standalone(String/*host*/, u16/*port*/, i64/*db*/),
    Cluster(),
    Sentinel {
        host: String,
        port: u16,
        node_password: Option<String>,
        master_group_name: String,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SslParams {
    pub client_cert: Option<ClientCert>,
    pub ca_cert: Option<String>,
    pub hostname_verify: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientCert {
    pub cert: String,
    pub key: String,
}