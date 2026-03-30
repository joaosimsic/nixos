use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

pub fn find_session_info_dir(session_name: &str) -> Result<PathBuf> {
    let home = std::env::var("HOME").context("Environment variable 'HOME' is not set")?;
    let zellij_cache = Path::new(&home).join(".cache/zellij");

    if !zellij_cache.exists() {
        anyhow::bail!(
            "Zellij cache directory not found at {}. Is Zellij installed?",
            zellij_cache.display()
        );
    }

    let session_path = fs::read_dir(&zellij_cache)
        .with_context(|| format!("Failed to read Zellij cache at {}", zellij_cache.display()))?
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().map(|t| t.is_dir()).unwrap_or(false))
        .find_map(|version_dir| {
            let potential_path = version_dir.path().join("session_info").join(session_name);
            if potential_path.exists() && potential_path.is_dir() {
                Some(potential_path)
            } else {
                None
            }
        });

    session_path.with_context(|| {
        format!(
            "Could not find active session data for '{}'. It may have been fully deleted.",
            session_name
        )
    })
}

pub fn resolve_path(path: &str) -> PathBuf {
    if path.starts_with('~') {
        let home = std::env::var("HOME").unwrap_or_default();
        PathBuf::from(path.replace('~', &home))
    } else {
        PathBuf::from(path)
    }
}
