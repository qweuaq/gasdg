//! Per-protocol share-link parsers (vmess://, vless://, trojan://, ss://).

use base64::{engine::general_purpose::STANDARD, Engine};
use serde_json::Value;
use uuid::Uuid;

use crate::models::{Protocol, ServerNode};

/// Parse `vmess://` base64-encoded JSON.
pub fn parse_vmess(uri: &str) -> Result<ServerNode, String> {
    let encoded = uri.strip_prefix("vmess://").unwrap_or(uri);
    let decoded = STANDARD
        .decode(encoded.trim())
        .map_err(|e| format!("vmess base64 error: {e}"))?;
    let json: Value = serde_json::from_slice(&decoded)
        .map_err(|e| format!("vmess json error: {e}"))?;

    Ok(ServerNode {
        id: Uuid::new_v4().to_string(),
        name: json["ps"].as_str().unwrap_or("VMess").to_string(),
        address: json["add"].as_str().unwrap_or("").to_string(),
        port: json["port"]
            .as_str()
            .and_then(|s| s.parse().ok())
            .or_else(|| json["port"].as_u64().map(|v| v as u16))
            .unwrap_or(0),
        protocol: Protocol::VMess,
        latency: None,
        settings: json.to_string(),
    })
}

/// Parse `vless://uuid@host:port?params#name`.
pub fn parse_vless(uri: &str) -> Result<ServerNode, String> {
    let stripped = uri.strip_prefix("vless://").unwrap_or(uri);
    let (user_host, name) = split_fragment(stripped);
    let (uuid_part, host_port_params) =
        user_host.split_once('@').ok_or("vless: missing @")?;
    let (host_port, _params) = split_query(host_port_params);
    let (host, port) = split_host_port(host_port)?;

    let settings = serde_json::json!({
        "uuid": uuid_part,
        "raw": uri,
    });

    Ok(ServerNode {
        id: Uuid::new_v4().to_string(),
        name: name.unwrap_or("VLESS").to_string(),
        address: host.to_string(),
        port,
        protocol: Protocol::VLess,
        latency: None,
        settings: settings.to_string(),
    })
}

/// Parse `trojan://password@host:port?params#name`.
pub fn parse_trojan(uri: &str) -> Result<ServerNode, String> {
    let stripped = uri.strip_prefix("trojan://").unwrap_or(uri);
    let (user_host, name) = split_fragment(stripped);
    let (password, host_port_params) =
        user_host.split_once('@').ok_or("trojan: missing @")?;
    let (host_port, _params) = split_query(host_port_params);
    let (host, port) = split_host_port(host_port)?;

    let settings = serde_json::json!({
        "password": password,
        "raw": uri,
    });

    Ok(ServerNode {
        id: Uuid::new_v4().to_string(),
        name: name.unwrap_or("Trojan").to_string(),
        address: host.to_string(),
        port,
        protocol: Protocol::Trojan,
        latency: None,
        settings: settings.to_string(),
    })
}

/// Parse `ss://base64(method:password)@host:port#name` or SIP002 format.
pub fn parse_shadowsocks(uri: &str) -> Result<ServerNode, String> {
    let stripped = uri.strip_prefix("ss://").unwrap_or(uri);
    let (main, name) = split_fragment(stripped);

    // SIP002: base64(method:password)@host:port
    if let Some((encoded, host_port)) = main.split_once('@') {
        let decoded = STANDARD
            .decode(encoded.trim())
            .map_err(|e| format!("ss base64 error: {e}"))?;
        let method_pass = String::from_utf8(decoded).map_err(|e| format!("ss utf8: {e}"))?;
        let (host_port_clean, _params) = split_query(host_port);
        let (host, port) = split_host_port(host_port_clean)?;

        let settings = serde_json::json!({
            "method_password": method_pass,
            "raw": uri,
        });

        return Ok(ServerNode {
            id: Uuid::new_v4().to_string(),
            name: name.unwrap_or("Shadowsocks").to_string(),
            address: host.to_string(),
            port,
            protocol: Protocol::Shadowsocks,
            latency: None,
            settings: settings.to_string(),
        });
    }

    // Legacy: base64(method:password@host:port)
    let decoded = STANDARD
        .decode(main.trim())
        .map_err(|e| format!("ss legacy base64: {e}"))?;
    let text = String::from_utf8(decoded).map_err(|e| format!("ss legacy utf8: {e}"))?;
    let (method_pass, host_port) = text
        .rsplit_once('@')
        .ok_or("ss legacy: missing @")?;
    let (host, port) = split_host_port(host_port)?;

    let settings = serde_json::json!({
        "method_password": method_pass,
        "raw": uri,
    });

    Ok(ServerNode {
        id: Uuid::new_v4().to_string(),
        name: name.unwrap_or("Shadowsocks").to_string(),
        address: host.to_string(),
        port,
        protocol: Protocol::Shadowsocks,
        latency: None,
        settings: settings.to_string(),
    })
}

// ─── Helpers ───

fn split_fragment(s: &str) -> (&str, Option<&str>) {
    match s.split_once('#') {
        Some((main, frag)) => (main, Some(frag)),
        None => (s, None),
    }
}

fn split_query(s: &str) -> (&str, Option<&str>) {
    match s.split_once('?') {
        Some((main, q)) => (main, Some(q)),
        None => (s, None),
    }
}

fn split_host_port(s: &str) -> Result<(&str, u16), String> {
    let (host, port_str) = s.rsplit_once(':').ok_or("missing port")?;
    let port: u16 = port_str.parse().map_err(|_| format!("invalid port: {port_str}"))?;
    Ok((host, port))
}
