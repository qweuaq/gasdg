//! Tauri IPC commands for server listing and latency testing.

use std::net::{TcpStream, SocketAddr};
use std::time::{Duration, Instant};
use std::collections::HashMap;

use tauri::State;
use crate::state::AppState;
use crate::models::ServerNode;

#[tauri::command]
pub async fn list_servers(
    state: State<'_, AppState>,
) -> Result<Vec<ServerNode>, String> {
    let subs = state.subscriptions.lock().await;
    let servers: Vec<ServerNode> = subs.iter().flat_map(|s| s.servers.clone()).collect();
    Ok(servers)
}

#[tauri::command]
pub async fn test_latency(server_id: String, state: State<'_, AppState>) -> Result<u32, String> {
    let subs = state.subscriptions.lock().await;
    let server = subs
        .iter()
        .flat_map(|s| &s.servers)
        .find(|s| s.id == server_id)
        .cloned()
        .ok_or("Server not found")?;
    drop(subs);

    tcp_ping(&server.address, server.port).await
}

#[tauri::command]
pub async fn test_all_latencies(
    state: State<'_, AppState>,
) -> Result<HashMap<String, u32>, String> {
    let subs = state.subscriptions.lock().await;
    let servers: Vec<ServerNode> = subs.iter().flat_map(|s| s.servers.clone()).collect();
    drop(subs);

    let mut results = HashMap::new();
    for srv in &servers {
        match tcp_ping(&srv.address, srv.port).await {
            Ok(ms) => { results.insert(srv.id.clone(), ms); }
            Err(_) => { /* skip unreachable servers */ }
        }
    }
    Ok(results)
}

#[tauri::command]
pub async fn select_fastest_server(
    state: State<'_, AppState>,
) -> Result<String, String> {
    let latencies = test_all_latencies(state).await?;
    latencies
        .into_iter()
        .min_by_key(|(_id, ms)| *ms)
        .map(|(id, _)| id)
        .ok_or("No reachable servers".into())
}

/// Simple TCP connect latency measurement.
async fn tcp_ping(host: &str, port: u16) -> Result<u32, String> {
    let addr_str = format!("{host}:{port}");
    let addr: SocketAddr = addr_str
        .parse()
        .or_else(|_| {
            // Attempt DNS resolution for hostnames.
            use std::net::ToSocketAddrs;
            addr_str
                .to_socket_addrs()
                .map_err(|e| e.to_string())?
                .next()
                .ok_or_else(|| "DNS resolution failed".to_string())
        })?;

    let start = Instant::now();
    tokio::task::spawn_blocking(move || {
        TcpStream::connect_timeout(&addr, Duration::from_secs(5))
    })
    .await
    .map_err(|e| e.to_string())?
    .map_err(|e| e.to_string())?;

    Ok(start.elapsed().as_millis() as u32)
}
