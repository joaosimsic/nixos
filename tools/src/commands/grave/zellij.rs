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

pub fn kill_session(name: &str) {
    let status = Command::new("zellij")
        .args(["kill-session", name])
        .status()
        .expect("Failed to kill session");

    if !status.success() {
        eprintln!("Error: Could not kill session '{}'", name);
    }
}

pub fn attach_session(name: &str) {
    let _ = Command::new("zellij").args(["attach", name]).status();
}
