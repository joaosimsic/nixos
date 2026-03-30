use directories::ProjectDirs;
use kdl::KdlDocument;
use std::fs;
use std::path::{Path, PathBuf};

pub struct SessionMetadata {
    pub cwd: Option<String>,
    pub tabs: usize,
}

pub fn get_metadata(session_name: &str) -> SessionMetadata {
    let cache_dir = get_zellij_cache_path(session_name);
    let mut metadata = SessionMetadata { cwd: None, tabs: 0 };

    if let Some(path) = cache_dir {
        let metadata_path = path.join("session-metadata.kdl");
        if let Ok(content) = fs::read_to_string(metadata_path) {
            if let Ok(doc) = content.parse::<KdlDocument>() {
                metadata.cwd = extract_cwd(&doc);
            }
        }
    }
    metadata
}

fn get_zellij_cache_path(session_name: &str) -> Option<PathBuf> {
    ProjectDirs::from("", "", "zellij")
        .map(|dirs| dirs.cache_dir().join(session_name))
        .filter(|path| path.exists())
}

fn extract_cwd(doc: &KdlDocument) -> Option<String> {
    doc.nodes()
        .iter()
        .find(|n| n.name().value() == "cwd")
        .and_then(|n| n.entries().first())
        .map(|e| e.value().to_string().replace('"', ""))
}

pub fn clean_nix_path(path: &str) -> String {
    if path.starts_with("/nix/store/") {
        let parts: Vec<&str> = path.split('/').collect();
        if parts.len() > 4 {
            return parts[4..].join("/");
        }
    }
    path.to_string()
}
