//! System proxy configuration (HTTP/SOCKS5).
//!
//! Uses platform-specific commands to set and unset the system proxy.

use std::process::Command as StdCommand;

/// Enable the system HTTP proxy on the current platform.
pub fn set_system_proxy(http_port: u16, socks_port: u16) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        set_windows_proxy(http_port)?;
    }
    #[cfg(target_os = "macos")]
    {
        set_macos_proxy(http_port, socks_port)?;
    }
    #[cfg(target_os = "linux")]
    {
        set_linux_proxy(http_port, socks_port)?;
    }
    log::info!("System proxy enabled (HTTP:{http_port}, SOCKS5:{socks_port})");
    Ok(())
}

/// Disable the system proxy.
pub fn unset_system_proxy() -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        unset_windows_proxy()?;
    }
    #[cfg(target_os = "macos")]
    {
        unset_macos_proxy()?;
    }
    #[cfg(target_os = "linux")]
    {
        unset_linux_proxy()?;
    }
    log::info!("System proxy disabled");
    Ok(())
}

// ─── Platform implementations ───

#[cfg(target_os = "windows")]
fn set_windows_proxy(http_port: u16) -> Result<(), String> {
    StdCommand::new("reg")
        .args([
            "add",
            r"HKCU\Software\Microsoft\Windows\CurrentVersion\Internet Settings",
            "/v", "ProxyEnable", "/t", "REG_DWORD", "/d", "1", "/f",
        ])
        .output()
        .map_err(|e| e.to_string())?;
    StdCommand::new("reg")
        .args([
            "add",
            r"HKCU\Software\Microsoft\Windows\CurrentVersion\Internet Settings",
            "/v", "ProxyServer", "/t", "REG_SZ",
            "/d", &format!("127.0.0.1:{http_port}"), "/f",
        ])
        .output()
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(target_os = "windows")]
fn unset_windows_proxy() -> Result<(), String> {
    StdCommand::new("reg")
        .args([
            "add",
            r"HKCU\Software\Microsoft\Windows\CurrentVersion\Internet Settings",
            "/v", "ProxyEnable", "/t", "REG_DWORD", "/d", "0", "/f",
        ])
        .output()
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(target_os = "macos")]
fn set_macos_proxy(http_port: u16, socks_port: u16) -> Result<(), String> {
    let service = "Wi-Fi";
    for (proto, port) in [("webproxy", http_port), ("socksfirewallproxy", socks_port)] {
        StdCommand::new("networksetup")
            .args([
                &format!("-set{proto}"),
                service,
                "127.0.0.1",
                &port.to_string(),
            ])
            .output()
            .map_err(|e| e.to_string())?;
        StdCommand::new("networksetup")
            .args([
                &format!("-set{proto}state"),
                service,
                "on",
            ])
            .output()
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[cfg(target_os = "macos")]
fn unset_macos_proxy() -> Result<(), String> {
    let service = "Wi-Fi";
    for proto in ["webproxy", "socksfirewallproxy"] {
        StdCommand::new("networksetup")
            .args([
                &format!("-set{proto}state"),
                service,
                "off",
            ])
            .output()
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[cfg(target_os = "linux")]
fn set_linux_proxy(http_port: u16, socks_port: u16) -> Result<(), String> {
    let run = |args: &[&str]| -> Result<(), String> {
        StdCommand::new(args[0])
            .args(&args[1..])
            .output()
            .map_err(|e| e.to_string())?;
        Ok(())
    };

    let http_schema = "org.gnome.system.proxy.http";
    let socks_schema = "org.gnome.system.proxy.socks";
    let http_port_str = http_port.to_string();
    let socks_port_str = socks_port.to_string();

    run(&["gsettings", "set", "org.gnome.system.proxy", "mode", "manual"])?;
    run(&["gsettings", "set", http_schema, "host", "127.0.0.1"])?;
    run(&["gsettings", "set", http_schema, "port", &http_port_str])?;
    run(&["gsettings", "set", socks_schema, "host", "127.0.0.1"])?;
    run(&["gsettings", "set", socks_schema, "port", &socks_port_str])?;

    Ok(())
}

#[cfg(target_os = "linux")]
fn unset_linux_proxy() -> Result<(), String> {
    StdCommand::new("gsettings")
        .args(["set", "org.gnome.system.proxy", "mode", "none"])
        .output()
        .map_err(|e| e.to_string())?;
    Ok(())
}
