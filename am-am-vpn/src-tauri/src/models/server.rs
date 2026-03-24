use serde::{Deserialize, Serialize};

/// Supported proxy protocols.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Protocol {
    VMess,
    VLess,
    Trojan,
    Shadowsocks,
}

/// Represents a single proxy server parsed from a subscription.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerNode {
    pub id: String,
    pub name: String,
    pub address: String,
    pub port: u16,
    pub protocol: Protocol,
    /// Latency in milliseconds, `None` when untested.
    pub latency: Option<u32>,
    /// Protocol-specific settings serialised as a JSON string.
    pub settings: String,
}

/// A subscription source with its associated servers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    pub id: String,
    pub url: String,
    pub name: String,
    pub updated_at: String,
    pub servers: Vec<ServerNode>,
}

/// Current connection state exposed to the UI.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionState {
    pub status: ConnectionStatus,
    pub server_id: Option<String>,
    pub connected_since: Option<String>,
    pub download_bytes: u64,
    pub upload_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ConnectionStatus {
    Disconnected,
    Connecting,
    Connected,
    Error,
}

/// Application settings persisted on disk.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub proxy_mode: ProxyMode,
    pub socks_port: u16,
    pub http_port: u16,
    pub dns_servers: Vec<String>,
    pub auto_connect: bool,
    pub auto_select_fastest: bool,
    pub subscription_auto_update: bool,
    pub update_interval_hours: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ProxyMode {
    System,
    Tun,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            proxy_mode: ProxyMode::System,
            socks_port: 10808,
            http_port: 10809,
            dns_servers: vec![
                "1.1.1.1".into(),
                "8.8.8.8".into(),
            ],
            auto_connect: false,
            auto_select_fastest: false,
            subscription_auto_update: true,
            update_interval_hours: 12,
        }
    }
}

impl Default for ConnectionState {
    fn default() -> Self {
        Self {
            status: ConnectionStatus::Disconnected,
            server_id: None,
            connected_since: None,
            download_bytes: 0,
            upload_bytes: 0,
        }
    }
}

/// A single log entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: String,
    pub level: String,
    pub message: String,
}
