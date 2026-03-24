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

impl AppState {
    pub fn new(data_dir: std::path::PathBuf) -> Self {
        let storage = EncryptedStorage::new(data_dir.join("encrypted"), "am-am-vpn-default-key");

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
