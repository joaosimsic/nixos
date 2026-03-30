use crate::commands::grave::parser;
use anyhow::Result;
use colored::*;
use serde_json::Value;
use std::io::IsTerminal;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn fzf_colors_main() -> String {
    let path = match crate::amber_dir::palette_json_path() {
        Ok(p) => p,
        Err(_) => return fzf_colors_main_fallback(),
    };
    let Ok(raw) = std::fs::read_to_string(&path) else {
        return fzf_colors_main_fallback();
    };
    let Ok(v) = serde_json::from_str::<Value>(&raw) else {
        return fzf_colors_main_fallback();
    };
    let dim = v.get("DIM").and_then(|x| x.as_str()).unwrap_or("586e75");
    let base = v.get("BASE").and_then(|x| x.as_str()).unwrap_or("586e75");
    let black = v.get("BLACK").and_then(|x| x.as_str()).unwrap_or("073642");
    let hl = v
        .get("ERROR_BRIGHT")
        .or_else(|| v.get("YELLOW_BRIGHT"))
        .and_then(|x| x.as_str())
        .unwrap_or("cb4b16");
    format!(
        "label:#{dim},border:#{dim},prompt:#{dim},fg+:#{black},bg+:#{base},hl:#{hl},hl+:#ffffff,separator:#{dim},pointer:#{dim}"
    )
}

fn fzf_colors_main_fallback() -> String {
    "label:#586e75,border:#586e75,prompt:#586e75,fg+:#073642,bg+:#586e75,hl:#cb4b16,hl+:#ffffff,separator:#586e75,pointer:#586e75".to_string()
}

fn fzf_colors_disabled() -> String {
    let path = match crate::amber_dir::palette_json_path() {
        Ok(p) => p,
        Err(_) => return fzf_colors_disabled_fallback(),
    };
    let Ok(raw) = std::fs::read_to_string(&path) else {
        return fzf_colors_disabled_fallback();
    };
    let Ok(v) = serde_json::from_str::<Value>(&raw) else {
        return fzf_colors_disabled_fallback();
    };
    let dim = v.get("DIM").and_then(|x| x.as_str()).unwrap_or("586e75");
    format!("label:#{dim},border:#{dim},fg:#{dim},fg+:#{dim},bg:-1,bg+:-1,gutter:-1,pointer:-1,header:#{dim}")
}

fn fzf_colors_disabled_fallback() -> String {
    "label:#586e75,border:#586e75,fg:#586e75,fg+:#586e75,bg:-1,bg+:-1,gutter:-1,pointer:-1,header:#586e75".to_string()
}

fn amber_exe() -> PathBuf {
    std::env::current_exe().unwrap_or_else(|_| PathBuf::from("amber"))
}

pub fn run_picker(exclude_current: bool, fullscreen: bool) -> Result<Option<String>> {
    if !std::io::stdin().is_terminal() || !std::io::stdout().is_terminal() {
        anyhow::bail!("interactive picker requires a TTY");
    }

    let margin = if fullscreen { "0%,0%" } else { "15%,15%" };
    let lines = super::list_display_lines(exclude_current)?;
    let input_data = lines.join("\n");

    let exe = amber_exe();
    let exe_str = exe.to_string_lossy();
    let list_suffix = if exclude_current {
        " --exclude-current"
    } else {
        ""
    };
    let delete_cmd = format!(
        "execute-silent({} grave delete {{1}})+reload-sync({} grave list-lines{})",
        shell_quote(&exe_str),
        shell_quote(&exe_str),
        list_suffix
    );
    let preview_cmd = format!("{} grave preview {{1}}", shell_quote(&exe_str));

    if lines.is_empty() {
        let disabled_colors = fzf_colors_disabled();
        let mut child = Command::new("fzf")
            .arg("--disabled")
            .arg("--layout=reverse")
            .arg("--border=sharp")
            .arg("--border-label= Grave ")
            .arg(format!("--margin={}", margin))
            .arg("--no-info")
            .arg("--pointer=")
            .arg(format!("--color={}", disabled_colors))
            .arg("--header=  Esc: close")
            .arg("--bind=enter:abort,esc:abort")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()?;

        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(b"No sessions\n");
        }

        let _ = child.wait_with_output()?;
        return Ok(None);
    }

    let mut cmd = Command::new("fzf");
    cmd.arg("--ansi")
        .arg("--layout=reverse")
        .arg("--info=inline-right")
        .arg("--separator=─")
        .arg("--border=sharp")
        .arg("--border-label= Grave ")
        .arg("--prompt= ")
        .arg("--pointer=")
        .arg("--highlight-line")
        .arg(format!("--color={}", fzf_colors_main()))
        .arg("--header=  Enter: switch │ ctrl-d: delete │ Esc: close")
        .arg("--header-border=line")
        .arg("--delimiter=│")
        .arg(format!("--margin={}", margin))
        .arg("--with-nth=1..")
        .arg("--bind=j:down,k:up,h:first,l:last")
        .arg(format!("--bind=ctrl-d:{}", delete_cmd))
        .arg("--preview")
        .arg(preview_cmd)
        .arg("--preview-window=right:35%:wrap")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped());

    let mut child = cmd.spawn()?;

    if let Some(mut stdin) = child.stdin.take() {
        let _ = stdin.write_all(input_data.as_bytes());
    }

    let output = child.wait_with_output()?;
    if !output.status.success() {
        return Ok(None);
    }

    let selection = String::from_utf8_lossy(&output.stdout);
    let trimmed = selection.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }

    let name = trimmed
        .split('│')
        .next()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());

    Ok(name)
}

fn shell_quote(s: &str) -> String {
    if s.is_empty() {
        return "''".into();
    }
    format!("'{}'", s.replace('\'', "'\\''"))
}

pub fn render_preview(session_name: &str) -> Result<()> {
    let metadata = match parser::get_session_details(session_name) {
        Ok(m) => m,
        Err(_) => {
            println!("{}", "Could not load metadata".red());
            return Ok(());
        }
    };

    println!("{}", "Session Details".bold().underline());
    println!("{:<10} {}", "Name:".dimmed(), metadata.name.green());
    println!("{:<10} {}", "Path:".dimmed(), metadata.cwd);

    if let Some(branch) = &metadata.branch {
        if !branch.is_empty() {
            println!(
                "{:<10} {}",
                "Branch:".dimmed(),
                format!(" {}", branch).yellow()
            );
        }
    }

    println!("{:<10} {}", "Tabs:".dimmed(), metadata.tabs);

    if !metadata.commands.is_empty() {
        println!("\n{}", "Active Programs:".bold().dimmed());
        for cmd in &metadata.commands {
            println!("  {} {}", "»".blue(), cmd);
        }
    }
    Ok(())
}
