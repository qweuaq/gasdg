//! Xray-core binary download and extraction.
//!
//! Fetches the latest Xray-core release from GitHub and extracts the
//! platform-appropriate binary into the core directory.

use std::path::{Path, PathBuf};

/// GitHub release API URL for Xray-core.
const XRAY_RELEASE_URL: &str =
    "https://api.github.com/repos/XTLS/Xray-core/releases/latest";

/// Download and extract Xray-core if it is not already present.
pub async fn ensure_xray_core(core_dir: &Path) -> Result<PathBuf, String> {
    std::fs::create_dir_all(core_dir)
        .map_err(|e| format!("Failed to create core dir: {e}"))?;

    let bin_path = xray_binary_path(core_dir);
    if bin_path.exists() {
        log::info!("Xray-core binary found at {}", bin_path.display());
        return Ok(bin_path);
    }

    log::info!("Xray-core not found, downloading…");
    let asset_name = platform_asset_name();

    // Fetch latest release metadata.
    let client = reqwest::Client::builder()
        .user_agent("Am-AmVPN/2.0")
        .build()
        .map_err(|e| format!("HTTP client error: {e}"))?;

    let release: serde_json::Value = client
        .get(XRAY_RELEASE_URL)
        .send()
        .await
        .map_err(|e| format!("Release fetch error: {e}"))?
        .json()
        .await
        .map_err(|e| format!("Release JSON error: {e}"))?;

    let download_url = release["assets"]
        .as_array()
        .and_then(|assets| {
            assets
                .iter()
                .find(|a| {
                    a["name"]
                        .as_str()
                        .map(|n| n.contains(&asset_name))
                        .unwrap_or(false)
                })
                .and_then(|a| a["browser_download_url"].as_str())
        })
        .ok_or_else(|| format!("No matching asset found for platform: {asset_name}"))?
        .to_string();

    log::info!("Downloading Xray-core from {download_url}");

    let zip_bytes = client
        .get(&download_url)
        .send()
        .await
        .map_err(|e| format!("Download error: {e}"))?
        .bytes()
        .await
        .map_err(|e| format!("Download read error: {e}"))?;

    // Extract the archive.
    extract_zip(&zip_bytes, core_dir)?;

    // Ensure binary is executable on Unix.
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let perms = std::fs::Permissions::from_mode(0o755);
        std::fs::set_permissions(&bin_path, perms)
            .map_err(|e| format!("chmod error: {e}"))?;
    }

    if bin_path.exists() {
        log::info!("Xray-core installed at {}", bin_path.display());
        Ok(bin_path)
    } else {
        Err("Xray binary not found after extraction".into())
    }
}

fn xray_binary_path(core_dir: &Path) -> PathBuf {
    #[cfg(target_os = "windows")]
    let name = "xray.exe";
    #[cfg(not(target_os = "windows"))]
    let name = "xray";
    core_dir.join(name)
}

fn platform_asset_name() -> String {
    let os = if cfg!(target_os = "windows") {
        "windows"
    } else if cfg!(target_os = "macos") {
        "macos"
    } else {
        "linux"
    };

    let arch = if cfg!(target_arch = "x86_64") {
        "64"
    } else if cfg!(target_arch = "aarch64") {
        "arm64-v8a"
    } else {
        "32"
    };

    format!("Xray-{os}-{arch}")
}

fn extract_zip(data: &[u8], dest: &Path) -> Result<(), String> {
    use std::io::{Cursor, Read, Write};

    let reader = Cursor::new(data);
    let mut archive =
        zip::ZipArchive::new(reader).map_err(|e| format!("ZIP open error: {e}"))?;

    for i in 0..archive.len() {
        let mut file = archive
            .by_index(i)
            .map_err(|e| format!("ZIP entry error: {e}"))?;

        let name = file.name().to_string();
        // Only extract the xray binary and geodata files.
        if name.contains('/') && !name.ends_with('/') {
            continue; // skip nested directories
        }

        let out_path = dest.join(&name);
        if file.is_dir() {
            std::fs::create_dir_all(&out_path)
                .map_err(|e| format!("mkdir error: {e}"))?;
        } else {
            let mut buf = Vec::new();
            file.read_to_end(&mut buf)
                .map_err(|e| format!("ZIP read error: {e}"))?;
            let mut out_file = std::fs::File::create(&out_path)
                .map_err(|e| format!("File create error: {e}"))?;
            out_file
                .write_all(&buf)
                .map_err(|e| format!("File write error: {e}"))?;
        }
    }
    Ok(())
}
