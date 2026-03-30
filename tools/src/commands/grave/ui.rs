use crate::commands::grave::{parser, types::SessionMetadata, zellij};
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
        let metadata = parser::get_session_details(&session.name)
            .unwrap_or_else(|_| SessionMetadata::new(&session.name));

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
            metadata.cwd.dimmed(),
        );

        lines.push(line);
    }

    lines.join("\n")
}

pub fn start_selector() -> Option<String> {
    let input_data = generate_fzf_lines();

    let preview_cmd = "amber grave --preview {2}";

    let mut child = Command::new("fzf")
        .args([
            "--ansi",
            "--header",
            "Enter: Attach | Ctrl-D: Delete | Ctrl-R: Reload",
            "--bind",
            "ctrl-d:execute(amber grave --delete {2})+reload(amber grave --list)",
            "--bind",
            "ctrl-r:reload(amber grave --list)",
            "--preview",
            preview_cmd,
            "--preview-window",
            "right:35%:wrap",
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
        let _ = stdin.write_all(input_data.as_bytes());
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

pub fn render_preview(session_name: &str) {
    let metadata = match parser::get_session_details(session_name) {
        Ok(m) => m,
        Err(_) => {
            println!("{}", "Could not load metadata".red());
            return;
        }
    };

    println!("{}", "Session Details".bold().underline());
    println!("{:<10} {}", "Name:".dimmed(), metadata.name.green());
    println!("{:<10} {}", "Path:".dimmed(), metadata.cwd);

    if let Some(branch) = metadata.branch {
        println!(
            "{:<10} {}",
            "Branch:".dimmed(),
            format!(" {}", branch).yellow()
        );
    }

    println!("{:<10} {}", "Tabs:".dimmed(), metadata.tabs);

    if !metadata.commands.is_empty() {
        println!("\n{}", "Active Programs:".bold().dimmed());
        for cmd in metadata.commands {
            println!("  {} {}", "»".blue(), cmd);
        }
    }
}
