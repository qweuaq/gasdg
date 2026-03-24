use base64::{engine::general_purpose::STANDARD, Engine};
use serde_json::Value;
use uuid::Uuid;

use crate::models::{Protocol, ServerNode, Subscription};
use super::protocols;

/// Fetch and parse a subscription URL. Supports both base64-encoded lists
/// and JSON arrays.
pub async fn fetch_subscription(url: &str) -> Result<Subscription, String> {
    let body = reqwest::get(url)
        .await
        .map_err(|e| format!("HTTP error: {e}"))?
        .text()
        .await
        .map_err(|e| format!("Body read error: {e}"))?;

    let servers = parse_body(&body)?;

    Ok(Subscription {
        id: Uuid::new_v4().to_string(),
        url: url.to_string(),
        name: subscription_name(url),
        updated_at: chrono::Utc::now().to_rfc3339(),
        servers,
    })
}

/// Parse the raw subscription body into a list of servers.
pub fn parse_body(body: &str) -> Result<Vec<ServerNode>, String> {
    let trimmed = body.trim();

    // Try JSON array first.
    if trimmed.starts_with('[') {
        return parse_json_array(trimmed);
    }

    // Otherwise treat as base64-encoded line list.
    let decoded = STANDARD
        .decode(trimmed)
        .map_err(|e| format!("base64 decode error: {e}"))?;
    let text = String::from_utf8(decoded).map_err(|e| format!("UTF-8 error: {e}"))?;

    let mut servers = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        match parse_uri(line) {
            Ok(node) => servers.push(node),
            Err(e) => log::warn!("Skipping line: {e}"),
        }
    }
    Ok(servers)
}

/// Parse a single share-link URI (vmess://, vless://, trojan://, ss://).
pub fn parse_uri(uri: &str) -> Result<ServerNode, String> {
    if uri.starts_with("vmess://") {
        protocols::parse_vmess(uri)
    } else if uri.starts_with("vless://") {
        protocols::parse_vless(uri)
    } else if uri.starts_with("trojan://") {
        protocols::parse_trojan(uri)
    } else if uri.starts_with("ss://") {
        protocols::parse_shadowsocks(uri)
    } else {
        Err(format!("Unknown protocol in URI: {uri}"))
    }
}

fn parse_json_array(json: &str) -> Result<Vec<ServerNode>, String> {
    let arr: Vec<Value> = serde_json::from_str(json)
        .map_err(|e| format!("JSON parse error: {e}"))?;
    let mut servers = Vec::new();
    for val in arr {
        let node = ServerNode {
            id: Uuid::new_v4().to_string(),
            name: val["ps"].as_str().or(val["name"].as_str()).unwrap_or("unnamed").to_string(),
            address: val["add"].as_str().or(val["address"].as_str()).unwrap_or("").to_string(),
            port: val["port"].as_u64().unwrap_or(0) as u16,
            protocol: match val["protocol"].as_str().unwrap_or("vmess") {
                "vless" => Protocol::VLess,
                "trojan" => Protocol::Trojan,
                "ss" | "shadowsocks" => Protocol::Shadowsocks,
                _ => Protocol::VMess,
            },
            latency: None,
            settings: val.to_string(),
        };
        servers.push(node);
    }
    Ok(servers)
}

fn subscription_name(url: &str) -> String {
    url::Url::parse(url)
        .ok()
        .and_then(|u| u.host_str().map(String::from))
        .unwrap_or_else(|| "subscription".into())
}
