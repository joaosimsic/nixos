pub mod parser;
pub mod scanner;
pub mod types;
pub mod ui;
pub mod zellij;

use anyhow::Result;
use regex::Regex;
use std::path::Path;
use std::time::Duration;

use types::{SessionMetadata, SessionStatus};

const MAX_EXITED_SESSIONS: usize = 10;
const GRAVE_PID_FILE: &str = "/tmp/grave-pid";

pub fn build_session_display(details: &SessionMetadata) -> String {
    let cwd_branch = match &details.branch {
        Some(b) if !b.is_empty() => format!("{} ({})", details.cwd, b),
        _ => details.cwd.clone(),
    };

    let cmd_str = if details.commands.is_empty() {
        String::new()
    } else {
        details.commands.join(" ")
    };
    let tab_str = if details.tabs > 1 {
        format!("{} tabs", details.tabs)
    } else {
        String::new()
    };

    let mut info_parts: Vec<String> = Vec::new();
    if !cmd_str.is_empty() {
        info_parts.push(cmd_str);
    }
    if !tab_str.is_empty() {
        info_parts.push(tab_str);
    }

    if info_parts.is_empty() {
        cwd_branch
    } else {
        format!("{} │ {}", cwd_branch, info_parts.join(" │ "))
    }
}

pub fn list_display_lines(exclude_current: bool) -> Result<Vec<String>> {
    let sessions = zellij::list_sessions()?;
    let filtered: Vec<_> = if exclude_current {
        sessions
            .into_iter()
            .filter(|s| s.status != SessionStatus::Current)
            .collect()
    } else {
        sessions
    };

    let mut lines = Vec::new();
    for s in filtered {
        let details = parser::get_session_details(&s.name)?;
        let display = build_session_display(&details);
        lines.push(format!("{} │ {}", s.name, display));
    }
    Ok(lines)
}

pub fn list_sessions_print() -> Result<()> {
    let sessions = zellij::list_sessions()?;
    for s in sessions {
        let details = parser::get_session_details(&s.name)?;
        let display = build_session_display(&details);
        let status = match s.status {
            SessionStatus::Current => "current",
            SessionStatus::Detached => "detached",
            SessionStatus::Exited => "exited",
        };
        println!("{} [{}] {}", s.name, status, display);
    }
    Ok(())
}

pub fn print_list_display(exclude_current: bool, or_empty: bool) -> Result<()> {
    let lines = list_display_lines(exclude_current)?;
    if lines.is_empty() {
        if or_empty {
            println!("No sessions");
        }
    } else {
        println!("{}", lines.join("\n"));
    }
    Ok(())
}

fn cleanup_old_sessions() -> Result<()> {
    let sessions = zellij::list_sessions()?;
    let exited: Vec<_> = sessions
        .iter()
        .filter(|s| s.status == SessionStatus::Exited)
        .collect();

    if exited.len() > MAX_EXITED_SESSIONS {
        for session in exited.iter().skip(MAX_EXITED_SESSIONS) {
            zellij::delete_session_force(&session.name)?;
            if let Some(dir) = scanner::session_data_dir(&session.name) {
                let _ = std::fs::remove_dir_all(dir);
            }
        }
    }
    Ok(())
}

pub fn run_main(switch_flag: bool) -> Result<()> {
    cleanup_old_sessions()?;

    if !zellij::is_available() {
        anyhow::bail!("zellij is not available in PATH");
    }

    let inside = std::env::var("ZELLIJ").is_ok();
    if inside && !switch_flag {
        if zellij::launch_session_manager_plugin().is_ok() {
            return Ok(());
        }
        eprintln!("Grave: session-manager plugin unavailable, falling back to picker");
    }

    if let Some(name) = ui::run_picker(false, false)? {
        parser::fix_nix_paths(&name)?;
        std::thread::sleep(Duration::from_millis(50));
        if inside {
            zellij::switch_session(&name)?;
        } else {
            zellij::attach_session(&name)?;
        }
    }
    Ok(())
}

pub fn run_switch() -> Result<()> {
    if !zellij::is_available() {
        anyhow::bail!("zellij is not available in PATH");
    }
    if std::env::var("ZELLIJ").is_err() {
        anyhow::bail!("'amber grave switch' must run inside a zellij session");
    }

    let pid_path = Path::new(GRAVE_PID_FILE);
    if pid_path.exists() {
        if let Ok(pid_str) = std::fs::read_to_string(pid_path) {
            let pid = pid_str.trim();
            if !pid.is_empty() && process_looks_like_grave_switch(pid) {
                let _ = std::process::Command::new("kill").arg(pid).status();
            }
        }
        let _ = std::fs::remove_file(pid_path);
        std::process::exit(0);
    }

    std::fs::write(pid_path, format!("{}", std::process::id()))?;
    let result = ui::run_picker(true, true);
    let _ = std::fs::remove_file(pid_path);

    match result {
        Ok(Some(name)) if !name.is_empty() => {
            parser::fix_nix_paths(&name)?;
            std::thread::sleep(Duration::from_millis(50));
            zellij::switch_session(&name)?;
        }
        _ => {}
    }
    Ok(())
}

pub fn run_toggle(amber_exe: &Path) -> Result<()> {
    let layout = zellij::dump_layout()?;
    let re = Regex::new(r#"pane.*id=(\d+).*name="Grave""#).expect("valid regex");
    if let Some(caps) = re.captures(&layout) {
        if let Some(m) = caps.get(1) {
            if let Ok(id) = m.as_str().parse::<u32>() {
                zellij::close_pane(id)?;
                return Ok(());
            }
        }
    }
    zellij::run_floating_grave_switch(amber_exe)?;
    Ok(())
}

pub fn run_clean(keep: usize) -> Result<()> {
    let sessions = zellij::list_sessions()?;
    let exited: Vec<_> = sessions
        .iter()
        .filter(|s| s.status == SessionStatus::Exited)
        .collect();

    if exited.len() <= keep {
        println!(
            "Only {} exited sessions, nothing to clean.",
            exited.len()
        );
        println!("Note: Detached sessions are not deleted (may be attached elsewhere).");
        println!("Use 'amber grave kill' to delete all detached sessions.");
        return Ok(());
    }

    for session in exited.iter().skip(keep) {
        zellij::delete_session_force(&session.name)?;
        if let Some(dir) = scanner::session_data_dir(&session.name) {
            let _ = std::fs::remove_dir_all(dir);
        }
        println!("Deleted: {}", session.name);
    }

    let deleted = exited.len() - keep;
    println!(
        "Cleaned up {} exited sessions, kept {} most recent.",
        deleted, keep
    );
    Ok(())
}

pub fn run_kill(keep: usize) -> Result<()> {
    let sessions = zellij::list_sessions()?;
    let deletable: Vec<_> = sessions
        .iter()
        .filter(|s| {
            s.status == SessionStatus::Detached || s.status == SessionStatus::Exited
        })
        .collect();

    if deletable.len() <= keep {
        println!(
            "Only {} detached/exited sessions, nothing to kill.",
            deletable.len()
        );
        return Ok(());
    }

    for session in deletable.iter().skip(keep) {
        zellij::delete_session_force(&session.name)?;
        if let Some(dir) = scanner::session_data_dir(&session.name) {
            let _ = std::fs::remove_dir_all(dir);
        }
        println!("Killed: {}", session.name);
    }

    let deleted = deletable.len() - keep;
    println!("Killed {} sessions, kept {}.", deleted, keep);
    Ok(())
}

pub fn delete_session(name: &str) -> Result<()> {
    zellij::kill_session(name);
    zellij::delete_session_force(name)?;
    if let Some(dir) = scanner::session_data_dir(name) {
        let _ = std::fs::remove_dir_all(dir);
    }
    std::thread::sleep(Duration::from_millis(150));
    Ok(())
}

pub fn print_preview(session_name: &str) -> Result<()> {
    ui::render_preview(session_name)
}

fn process_looks_like_grave_switch(pid: &str) -> bool {
    let path = format!("/proc/{}/cmdline", pid);
    let Ok(bytes) = std::fs::read(path) else {
        return false;
    };
    let cmdline = String::from_utf8_lossy(&bytes).replace('\0', " ");
    cmdline.contains("grave") && cmdline.contains("switch")
}
