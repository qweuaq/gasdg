//! Xray-core process lifecycle management.

use std::path::PathBuf;
use std::process::Stdio;
use tokio::process::{Child, Command};
use tokio::io::{AsyncBufReadExt, BufReader};

/// Manages an Xray-core child process.
pub struct XrayProcess {
    child: Option<Child>,
    config_path: PathBuf,
}

impl XrayProcess {
    pub fn new(config_path: PathBuf) -> Self {
        Self {
            child: None,
            config_path,
        }
    }

    /// Start the Xray-core process with the given binary path.
    pub async fn start(&mut self, xray_bin: &PathBuf) -> Result<(), String> {
        if self.child.is_some() {
            return Err("Xray process is already running".into());
        }

        let child = Command::new(xray_bin)
            .arg("run")
            .arg("-config")
            .arg(&self.config_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .kill_on_drop(true)
            .spawn()
            .map_err(|e| format!("Failed to start xray: {e}"))?;

        self.child = Some(child);
        log::info!("Xray-core process started");
        Ok(())
    }

    /// Stop the running Xray-core process.
    pub async fn stop(&mut self) -> Result<(), String> {
        if let Some(mut child) = self.child.take() {
            child
                .kill()
                .await
                .map_err(|e| format!("Failed to kill xray: {e}"))?;
            log::info!("Xray-core process stopped");
        }
        Ok(())
    }

    /// Check if the process is still running.
    pub fn is_running(&mut self) -> bool {
        match &mut self.child {
            Some(child) => child.try_wait().ok().flatten().is_none(),
            None => false,
        }
    }

    /// Read available lines from stdout (non-blocking via tokio).
    pub async fn read_logs(&mut self) -> Vec<String> {
        let mut lines = Vec::new();
        if let Some(child) = &mut self.child {
            if let Some(stdout) = child.stdout.take() {
                let reader = BufReader::new(stdout);
                let mut line_stream = reader.lines();
                while let Ok(Some(line)) = line_stream.next_line().await {
                    lines.push(line);
                }
            }
        }
        lines
    }
}

impl Drop for XrayProcess {
    fn drop(&mut self) {
        if let Some(mut child) = self.child.take() {
            let _ = child.start_kill();
        }
    }
}
