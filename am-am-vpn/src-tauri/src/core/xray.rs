//! High-level Xray-core management: download, configure, start/stop.

use std::path::PathBuf;

use crate::models::{AppSettings, ServerNode};
use super::config::build_xray_config;
use super::process::XrayProcess;

/// Main Xray engine combining configuration generation with process lifecycle.
pub struct XrayEngine {
    data_dir: PathBuf,
    process: Option<XrayProcess>,
}

impl XrayEngine {
    pub fn new(data_dir: PathBuf) -> Self {
        Self {
            data_dir,
            process: None,
        }
    }

    /// Directory where Xray binary and configs live.
    fn core_dir(&self) -> PathBuf {
        self.data_dir.join("xray-core")
    }

    fn config_path(&self) -> PathBuf {
        self.core_dir().join("config.json")
    }

    fn xray_binary(&self) -> PathBuf {
        #[cfg(target_os = "windows")]
        let name = "xray.exe";
        #[cfg(not(target_os = "windows"))]
        let name = "xray";
        self.core_dir().join(name)
    }

    /// Ensure the xray-core directory exists.
    pub fn ensure_dirs(&self) -> Result<(), String> {
        std::fs::create_dir_all(self.core_dir())
            .map_err(|e| format!("Failed to create core dir: {e}"))
    }

    /// Check if the Xray binary is available.
    pub fn is_core_available(&self) -> bool {
        self.xray_binary().exists()
    }

    /// Write the JSON config for the selected server and start the process.
    pub async fn connect(
        &mut self,
        server: &ServerNode,
        settings: &AppSettings,
    ) -> Result<(), String> {
        self.ensure_dirs()?;

        // Build and write config.
        let config = build_xray_config(server, settings);
        let config_str = serde_json::to_string_pretty(&config)
            .map_err(|e| format!("Config serialize error: {e}"))?;
        std::fs::write(self.config_path(), &config_str)
            .map_err(|e| format!("Config write error: {e}"))?;

        // Start process.
        let mut proc = XrayProcess::new(self.config_path());
        proc.start(&self.xray_binary()).await?;
        self.process = Some(proc);
        Ok(())
    }

    /// Stop the running core process.
    pub async fn disconnect(&mut self) -> Result<(), String> {
        if let Some(mut proc) = self.process.take() {
            proc.stop().await?;
        }
        Ok(())
    }

    /// Returns true when the core process is alive.
    pub fn is_connected(&mut self) -> bool {
        self.process
            .as_mut()
            .map(|p| p.is_running())
            .unwrap_or(false)
    }
}
