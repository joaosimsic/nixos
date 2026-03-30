use anyhow::{Context, Result};
use kdl::{KdlDocument, KdlNode, KdlValue};
use std::fs;
use std::process::Command;

use crate::commands::grave::scanner;
use crate::commands::grave::types::SessionMetadata;

pub fn get_session_details(session_name: &str) -> Result<SessionMetadata> {
    let session_dir = scanner::find_session_info_dir(session_name)?;
    let mut details = SessionMetadata::new(session_name);

    let targets = ["session-metadata.kdl", "session-layout.kdl"];
    let found_path = targets
        .iter()
        .map(|f| session_dir.join(f))
        .find(|p| p.exists());

    if let Some(path) = found_path {
        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read {}", path.display()))?;
        
        let doc: KdlDocument = content.parse()
            .map_err(|e| anyhow::anyhow!("KDL Parse Error in {}: {}", path.display(), e))?;

        extract_info_recursive(doc.nodes(), &mut details);
        
        if details.cwd != "unknown" {
            details.branch = fetch_git_branch(&details.cwd);
        }
    }

    details.commands.sort();
    details.commands.dedup();
    Ok(details)
}

pub fn fix_nix_paths(session_name: &str) -> Result<()> {
    let session_dir = scanner::find_session_info_dir(session_name)?;
    
    for filename in ["session-layout.kdl", "session-metadata.kdl"] {
        let path = session_dir.join(filename);
        if !path.exists() { continue; }

        let content = fs::read_to_string(&path)?;
        if !content.contains("/nix/store/") { continue; }

        let mut doc: KdlDocument = content.parse()
            .map_err(|e| anyhow::anyhow!("Failed to parse KDL for Nix fix: {}", e))?;

        apply_nix_fixes(doc.nodes_mut());

        let tmp_path = path.with_extension("tmp");
        fs::write(&tmp_path, doc.to_string())?;
        fs::rename(&tmp_path, &path)?;
    }
    Ok(())
}

fn apply_nix_fixes(nodes: &mut Vec<KdlNode>) {
    for node in nodes {
        if let Some(cmd_entry) = node.entries_mut().iter_mut().find(|e| e.name().map(|n| n.value()) == Some("command")) {
            let val = cmd_entry.value().to_string().replace('"', "");
            if val.contains("/nix/store/") {
                let bin = if val.ends_with("/bin/nvim") { 
                    "nvim".to_string() 
                } else { 
                    val.split('/').last().unwrap_or(&val).to_string() 
                };
                *cmd_entry.value_mut() = KdlValue::String(bin);
            }
        }

        if let Some(children) = node.children_mut() {
            children.nodes_mut().retain(|n| {
                let is_nix_lua = n.name().value() == "args" && 
                    n.entries().iter().any(|e| e.value().to_string().contains("lua dofile"));
                !is_nix_lua
            });
            apply_nix_fixes(children.nodes_mut());
        }
    }
}

fn extract_info_recursive(nodes: &[KdlNode], details: &mut SessionMetadata) {
    for node in nodes {
        match node.name().value() {
            "tab" => details.tabs += 1,
            "cwd" => {
                if let Some(entry) = node.entries().first() {
                    details.cwd = entry.value().to_string().replace('"', "");
                }
            },
            "title" => {
                if let Some(entry) = node.entries().first() {
                    let val = entry.value().to_string().replace('"', "");
                    if val.starts_with('/') || val.starts_with('~') {
                        details.cwd = val;
                    }
                }
            },
            _ => {
                if let Some(cmd_entry) = node.entries().iter().find(|e| e.name().map(|n| n.value()) == Some("command")) {
                    let cmd = cmd_entry.value().to_string().replace('"', "");
                    let bin = cmd.split('/').last().unwrap_or(&cmd).to_string();
                    let ignored = ["nu", "bash", "zsh", "fish", "sh", "zellij"];
                    if !ignored.contains(&bin.as_str()) && !bin.is_empty() {
                        details.commands.push(bin);
                    }
                }
            }
        }
        if let Some(children) = node.children() {
            extract_info_recursive(children.nodes(), details);
        }
    }
}

fn fetch_git_branch(cwd: &str) -> Option<String> {
    let expanded = scanner::resolve_path(cwd);
    Command::new("git")
        .args(["-C", &expanded.to_string_lossy(), "rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .ok()
        .filter(|out| out.status.success())
        .map(|out| String::from_utf8_lossy(&out.stdout).trim().to_string())
}
