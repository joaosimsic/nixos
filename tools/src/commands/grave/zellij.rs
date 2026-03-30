use anyhow::{Context, Result};
use std::process::Command;

#[derive(Debug, Clone)]
pub struct ZellijSession {
    pub name: String,
    pub is_current: bool,
    pub is_exited: bool,
}

pub fn get_sessions() -> Vec<ZellijSession> {
    let output = Command::new("zellij")
        .args(["list-sessions", "--no-formatting"])
        .output()
        .expect("Failed to execute zellij");

    let stdout = String::from_utf8_lossy(&output.stdout);

    stdout
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| ZellijSession {
            name: parse_name(line),
            is_current: line.contains("(current)"),
            is_exited: line.contains("EXITED"),
        })
        .collect()
}

fn parse_name(line: &str) -> String {
    line.split_whitespace()
        .next()
        .unwrap_or_default()
        .to_string()
}

pub fn attach_session(name: &str) -> Result<()> {
    Command::new("zellij")
        .args(["attach", name])
        .status()
        .context(format!("Failed to attach to session '{}'", name))?;
    Ok(())
}

pub fn switch_session(name: &str) -> Result<()> {
    Command::new("zellij")
        .args(["action", "switch-session", name])
        .status()
        .context(format!("Failed to switch to session '{}'", name))?;
    Ok(())
}

pub fn delete_session_force(name: &str) -> Result<()> {
    Command::new("zellij")
        .args(["delete-session", "--force", name])
        .status()
        .context(format!("Failed to force delete session '{}'", name))?;
    Ok(())
}

pub fn kill_session(name: &str) -> Result<()> {
    let status = Command::new("zellij")
        .args(["kill-session", name])
        .status()
        .context("Failed to kill session")?;

    if !status.success() {
        anyhow::bail!("Error: Could not kill session '{}'", name);
    }
    Ok(())
}
