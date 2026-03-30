use crate::commands::grave::{parser, zellij};
use colored::*;
use std::io::Write;
use std::process::{Command, Stdio};

pub fn generate_fzf_lines() -> String {
    let sessions = zellij::get_sessions();

    if sessions.is_empty() {
        return format!("{} {}", "○".red(), "No sessions found".bold().dimmed());
    }

    let mut lines = Vec::new();
    for session in sessions {
        let metadata = parser::get_metadata(&session.name);
        let cwd = metadata.cwd.unwrap_or_else(|| "unknown".to_string());
        let clean_cwd = parser::clean_nix_path(&cwd);

        let status_icon = if session.is_current {
            "●".green()
        } else if session.is_exited {
            "○".red()
        } else {
            "◌".yellow()
        };

        let line = format!(
            "{} {:<15} | {:<30}",
            status_icon,
            session.name.bold(),
            clean_cwd.dimmed(),
        );

        lines.push(line);
    }

    lines.join("\n")
}

pub fn start_selector() -> Option<String> {
    let input_data = generate_fzf_lines();

    let mut child = Command::new("fzf")
        .args([
            "--ansi",
            "--header",
            "Grave | Tab: Preview | Ctrl-D: Kill",
            "--delimiter",
            "\\|",
            "--with-nth",
            "1",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .ok()?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin.write_all(input_data.as_bytes()).ok();
    }

    let output = child.wait_with_output().ok()?;
    if output.status.success() {
        let selection = String::from_utf8_lossy(&output.stdout);

        if selection.contains("No sessions found") {
            return None;
        }

        return selection.split_whitespace().nth(1).map(|s| s.to_string());
    }

    None
}
