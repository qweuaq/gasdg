//! Xray-core JSON configuration builder.
//!
//! Generates a valid Xray JSON config from a `ServerNode` and `AppSettings`.

use serde_json::{json, Value};

use crate::models::{AppSettings, Protocol, ProxyMode, ServerNode};

/// Build a complete Xray-core configuration for the given server.
pub fn build_xray_config(server: &ServerNode, settings: &AppSettings) -> Value {
    let inbounds = build_inbounds(settings);
    let outbounds = build_outbounds(server);
    let dns = build_dns(settings);
    let routing = build_routing(settings);

    json!({
        "log": {
            "loglevel": "warning"
        },
        "inbounds": inbounds,
        "outbounds": outbounds,
        "dns": dns,
        "routing": routing,
    })
}

fn build_inbounds(settings: &AppSettings) -> Vec<Value> {
    let mut inbounds = vec![
        json!({
            "tag": "socks-in",
            "port": settings.socks_port,
            "listen": "127.0.0.1",
            "protocol": "socks",
            "settings": {
                "auth": "noauth",
                "udp": true
            },
            "sniffing": {
                "enabled": true,
                "destOverride": ["http", "tls"]
            }
        }),
        json!({
            "tag": "http-in",
            "port": settings.http_port,
            "listen": "127.0.0.1",
            "protocol": "http",
            "settings": {}
        }),
    ];

    if matches!(settings.proxy_mode, ProxyMode::Tun) {
        inbounds.push(json!({
            "tag": "tun-in",
            "port": 0,
            "protocol": "dokodemo-door",
            "settings": {
                "network": "tcp,udp",
                "followRedirect": true
            },
            "sniffing": {
                "enabled": true,
                "destOverride": ["http", "tls"]
            }
        }));
    }

    inbounds
}

fn build_outbounds(server: &ServerNode) -> Vec<Value> {
    let proxy = match server.protocol {
        Protocol::VMess => build_vmess_outbound(server),
        Protocol::VLess => build_vless_outbound(server),
        Protocol::Trojan => build_trojan_outbound(server),
        Protocol::Shadowsocks => build_ss_outbound(server),
    };

    vec![
        proxy,
        json!({
            "tag": "direct",
            "protocol": "freedom",
            "settings": {}
        }),
        json!({
            "tag": "block",
            "protocol": "blackhole",
            "settings": {}
        }),
    ]
}

fn build_vmess_outbound(server: &ServerNode) -> Value {
    let raw: Value = serde_json::from_str(&server.settings).unwrap_or_default();
    json!({
        "tag": "proxy",
        "protocol": "vmess",
        "settings": {
            "vnext": [{
                "address": server.address,
                "port": server.port,
                "users": [{
                    "id": raw["id"].as_str().unwrap_or(""),
                    "alterId": raw["aid"].as_u64().unwrap_or(0),
                    "security": raw["scy"].as_str().unwrap_or("auto")
                }]
            }]
        },
        "streamSettings": stream_settings(&raw)
    })
}

fn build_vless_outbound(server: &ServerNode) -> Value {
    let raw: Value = serde_json::from_str(&server.settings).unwrap_or_default();
    json!({
        "tag": "proxy",
        "protocol": "vless",
        "settings": {
            "vnext": [{
                "address": server.address,
                "port": server.port,
                "users": [{
                    "id": raw["uuid"].as_str().unwrap_or(""),
                    "encryption": "none",
                    "flow": raw["flow"].as_str().unwrap_or("")
                }]
            }]
        },
        "streamSettings": stream_settings(&raw)
    })
}

fn build_trojan_outbound(server: &ServerNode) -> Value {
    let raw: Value = serde_json::from_str(&server.settings).unwrap_or_default();
    json!({
        "tag": "proxy",
        "protocol": "trojan",
        "settings": {
            "servers": [{
                "address": server.address,
                "port": server.port,
                "password": raw["password"].as_str().unwrap_or("")
            }]
        },
        "streamSettings": {
            "network": "tcp",
            "security": "tls",
            "tlsSettings": {
                "serverName": server.address
            }
        }
    })
}

fn build_ss_outbound(server: &ServerNode) -> Value {
    let raw: Value = serde_json::from_str(&server.settings).unwrap_or_default();
    let method_pass = raw["method_password"].as_str().unwrap_or("aes-256-gcm:");
    let parts: Vec<&str> = method_pass.splitn(2, ':').collect();
    let (method, password) = if parts.len() == 2 {
        (parts[0], parts[1])
    } else {
        ("aes-256-gcm", "")
    };

    json!({
        "tag": "proxy",
        "protocol": "shadowsocks",
        "settings": {
            "servers": [{
                "address": server.address,
                "port": server.port,
                "method": method,
                "password": password
            }]
        }
    })
}

fn stream_settings(raw: &Value) -> Value {
    let network = raw["net"].as_str().unwrap_or("tcp");
    let security = raw["tls"].as_str().unwrap_or("");

    let mut ss = json!({
        "network": network,
    });

    if security == "tls" {
        ss["security"] = json!("tls");
        ss["tlsSettings"] = json!({
            "serverName": raw["sni"].as_str().or(raw["host"].as_str()).unwrap_or(""),
            "allowInsecure": false
        });
    }

    match network {
        "ws" => {
            ss["wsSettings"] = json!({
                "path": raw["path"].as_str().unwrap_or("/"),
                "headers": {
                    "Host": raw["host"].as_str().unwrap_or("")
                }
            });
        }
        "grpc" => {
            ss["grpcSettings"] = json!({
                "serviceName": raw["path"].as_str().unwrap_or("")
            });
        }
        _ => {}
    }

    ss
}

fn build_dns(settings: &AppSettings) -> Value {
    json!({
        "servers": settings.dns_servers.iter().map(|s| json!(s)).collect::<Vec<_>>(),
        "queryStrategy": "UseIP"
    })
}

fn build_routing(_settings: &AppSettings) -> Value {
    let mut rules = vec![
        json!({
            "type": "field",
            "outboundTag": "direct",
            "domain": ["geosite:private"]
        }),
    ];

    // DNS leak protection: route all DNS through proxy.
    rules.push(json!({
        "type": "field",
        "outboundTag": "proxy",
        "port": "53"
    }));

    json!({
        "domainStrategy": "IPIfNonMatch",
        "rules": rules,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::AppSettings;

    #[test]
    fn test_vmess_config() {
        let server = ServerNode {
            id: "test".into(),
            name: "Test".into(),
            address: "1.2.3.4".into(),
            port: 443,
            protocol: Protocol::VMess,
            latency: None,
            settings: r#"{"id":"abc","aid":0,"net":"ws","path":"/ws","host":"example.com","tls":"tls","sni":"example.com"}"#.into(),
        };
        let cfg = build_xray_config(&server, &AppSettings::default());
        assert!(cfg["outbounds"][0]["protocol"] == "vmess");
    }
}
