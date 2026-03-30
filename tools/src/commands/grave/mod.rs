pub mod parser;
pub mod scanner;
pub mod types;
pub mod ui;
pub mod zellij;

use anyhow::Result;
use std::env;

const MAX_EXITED_SESSIONS: usize = 10;

pub fn run() -> Result<()> {
    cleanup_old_sessions()?;

    if let Some(selected_session) = ui::start_selector() {
        parser::fix_nix_paths(&selected_session)?;

        let is_inside_zellij = env::var("ZELLIJ").is_ok();

        if is_inside_zellij {
            zellij::switch_session(&selected_session)?;
        } else {
            zellij::attach_session(&selected_session)?;
        }
    }

    Ok(())
}

fn cleanup_old_sessions() -> Result<()> {
    let sessions = zellij::get_sessions();

    let exited: Vec<_> = sessions.iter().filter(|s| s.is_exited).collect();

    if exited.len() > MAX_EXITED_SESSIONS {
        for session in exited.iter().skip(MAX_EXITED_SESSIONS) {
            zellij::delete_session_force(&session.name)?;

            if let Ok(path) = scanner::find_session_info_dir(&session.name) {
                let _ = std::fs::remove_dir_all(path);
            }
        }
    }
    Ok(())
}
