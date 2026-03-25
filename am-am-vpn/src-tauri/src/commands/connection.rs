//! Tauri IPC commands for VPN connection management.

use tauri::State;
use crate::models::{AppSettings, ConnectionState, ConnectionStatus, LogEntry};
use crate::state::AppState;
use crate::proxy::system_proxy;

#[tauri::command]
pub async fn connect(
    server_id: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    // Find the server.
    let subs = state.subscriptions.lock().await;
    let server = subs
        .iter()
        .flat_map(|s| &s.servers)
        .find(|s| s.id == server_id)
        .cloned()
        .ok_or("Server not found")?;
    drop(subs);

    // Update status → connecting.
    {
        let mut conn = state.connection.lock().await;
        conn.status = ConnectionStatus::Connecting;
        conn.server_id = Some(server_id.clone());
    }

    let settings = state.settings.lock().await.clone();

    // Start Xray-core.
    let mut engine = state.engine.lock().await;
    if let Err(e) = engine.connect(&server, &settings).await {
        let mut conn = state.connection.lock().await;
        conn.status = ConnectionStatus::Error;
        state.push_log("error", &format!("Connect failed: {e}"));
        return Err(e);
    }

    // Set system proxy.
    if let Err(e) = system_proxy::set_system_proxy(settings.http_port, settings.socks_port) {
        log::warn!("System proxy error: {e}");
        state.push_log("warn", &format!("System proxy error: {e}"));
    }

    // Update status → connected.
    {
        let mut conn = state.connection.lock().await;
        conn.status = ConnectionStatus::Connected;
        conn.connected_since = Some(chrono::Utc::now().to_rfc3339());
    }

    state.push_log("info", &format!("Connected to {}", server.name));
    Ok(())
}

#[tauri::command]
pub async fn disconnect(state: State<'_, AppState>) -> Result<(), String> {
    let mut engine = state.engine.lock().await;
    engine.disconnect().await?;

    if let Err(e) = system_proxy::unset_system_proxy() {
        log::warn!("Unset proxy error: {e}");
    }

    let mut conn = state.connection.lock().await;
    *conn = ConnectionState::default();

    state.push_log("info", "Disconnected");
    Ok(())
}

#[tauri::command]
pub async fn get_connection_state(
    state: State<'_, AppState>,
) -> Result<ConnectionState, String> {
    let conn = state.connection.lock().await;
    Ok(conn.clone())
}

#[tauri::command]
pub async fn get_settings(state: State<'_, AppState>) -> Result<AppSettings, String> {
    let s = state.settings.lock().await;
    Ok(s.clone())
}

#[tauri::command]
pub async fn update_settings(
    settings: AppSettings,
    state: State<'_, AppState>,
) -> Result<(), String> {
    let mut s = state.settings.lock().await;
    *s = settings;
    Ok(())
}

#[tauri::command]
pub async fn get_logs(
    count: usize,
    state: State<'_, AppState>,
) -> Result<Vec<LogEntry>, String> {
    let logs = state.logs.lock().await;
    let start = if logs.len() > count { logs.len() - count } else { 0 };
    Ok(logs[start..].to_vec())
}
