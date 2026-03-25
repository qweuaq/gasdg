//! Shared application state managed by Tauri.

use tokio::sync::Mutex;

use crate::core::xray::XrayEngine;
use crate::models::{AppSettings, ConnectionState, LogEntry, Subscription};
use crate::storage::encrypted::EncryptedStorage;

pub struct AppState {
    pub subscriptions: Mutex<Vec<Subscription>>,
    pub connection: Mutex<ConnectionState>,
    pub settings: Mutex<AppSettings>,
    pub engine: Mutex<XrayEngine>,
    pub logs: Mutex<Vec<LogEntry>>,
    pub storage: EncryptedStorage,
}

/// Derive a per-machine encryption passphrase from the hostname and data
/// directory so that each installation gets a unique key.
fn derive_passphrase(data_dir: &std::path::Path) -> String {
    let host = hostname::get()
        .map(|h| h.to_string_lossy().to_string())
        .unwrap_or_else(|_| "am-am-vpn".to_string());
    format!("am-am-vpn:{}:{}", host, data_dir.display())
}

impl AppState {
    pub fn new(data_dir: std::path::PathBuf) -> Self {
        let passphrase = derive_passphrase(&data_dir);
        let storage = EncryptedStorage::new(data_dir.join("encrypted"), &passphrase);

        // Try to load persisted subscriptions.
        let subscriptions = storage
            .read_json::<Vec<Subscription>>("subscriptions.enc")
            .unwrap_or_default();

        let settings = storage
            .read_json::<AppSettings>("settings.enc")
            .unwrap_or_default();

        Self {
            subscriptions: Mutex::new(subscriptions),
            connection: Mutex::new(ConnectionState::default()),
            settings: Mutex::new(settings),
            engine: Mutex::new(XrayEngine::new(data_dir)),
            logs: Mutex::new(Vec::new()),
            storage,
        }
    }

    /// Persist subscriptions to encrypted storage.
    pub fn persist_subscriptions(&self, subs: &[Subscription]) -> Result<(), String> {
        self.storage.write_json("subscriptions.enc", &subs)
    }

    /// Persist settings to encrypted storage.
    pub fn persist_settings(&self, settings: &AppSettings) -> Result<(), String> {
        self.storage.write_json("settings.enc", settings)
    }

    /// Push a log entry.
    pub fn push_log(&self, level: &str, message: &str) {
        let entry = LogEntry {
            timestamp: chrono::Utc::now().to_rfc3339(),
            level: level.to_string(),
            message: message.to_string(),
        };
        // Best-effort, non-blocking.
        if let Ok(mut logs) = self.logs.try_lock() {
            logs.push(entry);
            // Keep a reasonable buffer.
            if logs.len() > 2000 {
                logs.drain(..1000);
            }
        }
    }
}
