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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_vmess() {
        // {"v":"2","ps":"Test","add":"1.2.3.4","port":"443","id":"abc-def","aid":"0","net":"ws","type":"none","host":"example.com","path":"/ws","tls":"tls"}
        let json = r#"{"v":"2","ps":"Test VMess","add":"1.2.3.4","port":"443","id":"abc-def","aid":"0","net":"ws","host":"example.com","path":"/ws","tls":"tls"}"#;
        let encoded = base64::engine::general_purpose::STANDARD.encode(json);
        let uri = format!("vmess://{encoded}");

        let node = parse_vmess(&uri).unwrap();
        assert_eq!(node.protocol, Protocol::VMess);
        assert_eq!(node.name, "Test VMess");
        assert_eq!(node.address, "1.2.3.4");
        assert_eq!(node.port, 443);
    }

    #[test]
    fn test_parse_vless() {
        let uri = "vless://uuid-123@example.com:443?security=tls&type=ws#My%20VLESS";
        let node = parse_vless(uri).unwrap();
        assert_eq!(node.protocol, Protocol::VLess);
        assert_eq!(node.address, "example.com");
        assert_eq!(node.port, 443);
        assert_eq!(node.name, "My%20VLESS");
    }

    #[test]
    fn test_parse_trojan() {
        let uri = "trojan://mypassword@server.net:8443?sni=server.net#Trojan-Server";
        let node = parse_trojan(uri).unwrap();
        assert_eq!(node.protocol, Protocol::Trojan);
        assert_eq!(node.address, "server.net");
        assert_eq!(node.port, 8443);
        assert_eq!(node.name, "Trojan-Server");
    }

    #[test]
    fn test_parse_shadowsocks_sip002() {
        // SIP002: ss://base64(method:password)@host:port#name
        let method_pass = "aes-256-gcm:mypassword";
        let encoded = base64::engine::general_purpose::STANDARD.encode(method_pass);
        let uri = format!("ss://{encoded}@1.2.3.4:8388#My-SS");
        let node = parse_shadowsocks(&uri).unwrap();
        assert_eq!(node.protocol, Protocol::Shadowsocks);
        assert_eq!(node.address, "1.2.3.4");
        assert_eq!(node.port, 8388);
        assert_eq!(node.name, "My-SS");
    }

    #[test]
    fn test_parse_vless_missing_at_fails() {
        let uri = "vless://uuid-no-host";
        assert!(parse_vless(uri).is_err());
    }

    #[test]
    fn test_parse_trojan_missing_at_fails() {
        let uri = "trojan://password-no-host";
        assert!(parse_trojan(uri).is_err());
    }

    #[test]
    fn test_split_host_port() {
        let (host, port) = split_host_port("example.com:443").unwrap();
        assert_eq!(host, "example.com");
        assert_eq!(port, 443);
    }

    #[test]
    fn test_split_host_port_invalid() {
        assert!(split_host_port("example.com:invalid").is_err());
        assert!(split_host_port("noport").is_err());
    }
}
