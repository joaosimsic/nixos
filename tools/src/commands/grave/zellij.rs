use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::commands::grave::types::{SessionInfo, SessionStatus};

pub fn is_available() -> bool {
    Command::new("zellij").arg("--version").output().is_ok()
}

pub fn list_sessions() -> Result<Vec<SessionInfo>> {
    let output = Command::new("zellij")
        .args(["list-sessions", "--no-formatting"])
        .output()
        .context("Failed to execute 'zellij list-sessions --no-formatting'")?;
    if !output.status.success() {
        anyhow::bail!(
            "zellij list-sessions failed: {}",
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let sessions = stdout
        .lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| SessionInfo {
            name: parse_name(line),
            status: parse_status(line),
        })
        .collect();
    Ok(sessions)
}

fn parse_name(line: &str) -> String {
    line.split_whitespace()
        .next()
        .unwrap_or_default()
        .to_string()
}

fn parse_status(line: &str) -> SessionStatus {
    if line.contains("current") {
        SessionStatus::Current
    } else if line.contains("EXITED") {
        SessionStatus::Exited
    } else {
        SessionStatus::Detached
    }
}

pub fn attach_session(name: &str) -> Result<()> {
    let status = Command::new("zellij")
        .args(["attach", name])
        .status()
        .context(format!("Failed to run zellij attach for '{}'", name))?;
    if !status.success() {
        anyhow::bail!("zellij attach failed for '{}'", name);
    }
    Ok(())
}

pub fn switch_session(name: &str) -> Result<()> {
    let status = Command::new("zellij")
        .args(["action", "switch-session", name])
        .status()
        .context(format!("Failed to run zellij switch-session for '{}'", name))?;
    if !status.success() {
        anyhow::bail!("zellij switch-session failed for '{}'", name);
    }
    Ok(())
}

pub fn delete_session_force(name: &str) -> Result<()> {
    let _ = Command::new("zellij")
        .args(["delete-session", "--force", name])
        .status();
    Ok(())
}

pub fn kill_session(name: &str) {
    let _ = Command::new("zellij")
        .args(["kill-session", name])
        .status();
}

pub fn launch_session_manager_plugin() -> Result<()> {
    let status = Command::new("zellij")
        .args([
            "action",
            "launch-or-focus-plugin",
            "session-manager",
            "--floating",
            "--move-to-focused-tab",
        ])
        .status()
        .context("Failed to launch session-manager plugin")?;
    if !status.success() {
        anyhow::bail!("launch-or-focus-plugin session-manager failed");
    }
    Ok(())
}

pub fn dump_layout() -> Result<String> {
    let output = Command::new("zellij")
        .args(["action", "dump-layout"])
        .output()
        .context("Failed to dump zellij layout")?;
    if !output.status.success() {
        anyhow::bail!("dump-layout failed");
    }
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

pub fn close_pane(pane_id: u32) -> Result<()> {
    let status = Command::new("zellij")
        .args(["action", "close-pane", "--pane-id", &pane_id.to_string()])
        .status()
        .context("Failed to close pane")?;
    if !status.success() {
        anyhow::bail!("close-pane failed for pane {}", pane_id);
    }
    Ok(())
}

pub fn run_floating_grave_switch(amber_exe: &std::path::Path) -> Result<()> {
    let status = Command::new("zellij")
        .arg("run")
        .arg("--floating")
        .arg("--close-on-exit")
        .arg("--name")
        .arg("Grave")
        .arg("--")
        .arg(amber_exe)
        .arg("grave")
        .arg("switch")
        .status()
        .context("Failed to spawn Grave switch pane")?;
    if !status.success() {
        anyhow::bail!("zellij run Grave switch failed");
    }
    Ok(())
}

pub fn start_session_default(name: &str) -> Result<()> {
    let status = Command::new("zellij")
        .args(["--session", name, "--layout", "default"])
        .status()
        .context(format!("Failed to run zellij for session '{}'", name))?;
    if !status.success() {
        anyhow::bail!("zellij failed to start session '{}'", name);
    }
    Ok(())
}

pub fn reattach_marker_path() -> PathBuf {
    let file_name = format!("amber-zellij-reattach-{}.txt", current_uid_tag());
    if let Ok(runtime) = std::env::var("XDG_RUNTIME_DIR") {
        return Path::new(&runtime).join(file_name);
    }
    std::env::temp_dir().join(file_name)
}

fn current_uid_tag() -> String {
    std::env::var("UID")
        .ok()
        .filter(|v| !v.trim().is_empty())
        .unwrap_or_else(|| "nouid".to_string())
}
