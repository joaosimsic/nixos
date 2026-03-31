use std::fs;
use std::path::{Path, PathBuf};

pub fn session_data_dir(session_name: &str) -> Option<PathBuf> {
    let zellij_cache = zellij_cache_dir()?;
    if !zellij_cache.is_dir() {
        return None;
    }

    let contract = zellij_cache
        .join("contract_version_1")
        .join("session_info")
        .join(session_name);
    if contract.is_dir() {
        return Some(contract);
    }

    fs::read_dir(&zellij_cache).ok()?.filter_map(|e| e.ok()).find_map(|entry| {
        if !entry.file_type().ok()?.is_dir() {
            return None;
        }
        let path = entry.path().join("session_info").join(session_name);
        if path.is_dir() {
            Some(path)
        } else {
            None
        }
    })
}

fn zellij_cache_dir() -> Option<PathBuf> {
    if let Ok(cache_home) = std::env::var("XDG_CACHE_HOME") {
        return Some(Path::new(&cache_home).join("zellij"));
    }

    let home = std::env::var("HOME").ok()?;
    Some(Path::new(&home).join(".cache/zellij"))
}

pub fn resolve_path(path: &str) -> PathBuf {
    if let Some(rest) = path.strip_prefix('~') {
        let home = std::env::var("HOME").unwrap_or_default();
        PathBuf::from(home + rest)
    } else {
        PathBuf::from(path)
    }
}

