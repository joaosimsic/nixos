use anyhow::{Context, Result};
use kdl::{KdlDocument, KdlNode};
use regex::Regex;
use std::fs;
use std::process::Command;

use crate::commands::grave::scanner;
use crate::commands::grave::types::SessionMetadata;

pub fn get_session_details(session_name: &str) -> Result<SessionMetadata> {
    let Some(session_dir) = scanner::session_data_dir(session_name) else {
        return Ok(SessionMetadata::new(session_name));
    };

    let mut details = SessionMetadata::new(session_name);

    let targets = ["session-metadata.kdl", "session-layout.kdl"];
    let found_path = targets
        .iter()
        .map(|f| session_dir.join(f))
        .find(|p| p.exists());

    if let Some(path) = found_path {
        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read {}", path.display()))?;

        match content.parse::<KdlDocument>() {
            Ok(doc) => extract_info_recursive(doc.nodes(), &mut details),
            Err(_) => extract_info_fallback(&content, &mut details),
        }

        if details.cwd != "unknown" {
            details.branch = fetch_git_branch(&details.cwd);
        }
    }

    normalize_cwd_home(&mut details);
    details.commands.sort();
    details.commands.dedup();
    Ok(details)
}

fn normalize_cwd_home(details: &mut SessionMetadata) {
    if let Ok(home) = std::env::var("HOME") {
        if details.cwd.starts_with(&home) && home != "/" {
            details.cwd = format!("~{}", &details.cwd[home.len()..]);
        }
    }
}

/// Matches the Nushell `fix-nix-paths` sed pipeline on raw KDL text.
pub fn fix_nix_paths(session_name: &str) -> Result<()> {
    let Some(session_dir) = scanner::session_data_dir(session_name) else {
        return Ok(());
    };

    let re_nvim = Regex::new(r#"command "/nix/store/[^"]*/bin/nvim""#).expect("valid regex");
    let re_nix_bin = Regex::new(r"/nix/store/[^/]*/bin/").expect("valid regex");

    for filename in ["session-layout.kdl", "session-metadata.kdl"] {
        let path = session_dir.join(filename);
        if !path.exists() {
            continue;
        }

        let content = fs::read_to_string(&path)?;
        if !content.contains("/nix/store/") {
            continue;
        }

        let mut s = re_nvim
            .replace_all(&content, r#"command="nvim""#)
            .into_owned();

        let mut lines: Vec<&str> = Vec::new();
        for line in s.lines() {
            if line.contains(r#"args "--cmd" "lua dofile"#) {
                continue;
            }
            lines.push(line);
        }
        s = lines.join("\n");
        if content.ends_with('\n') && !s.ends_with('\n') {
            s.push('\n');
        }

        s = re_nix_bin.replace_all(&s, "").into_owned();

        let tmp_path = path.with_extension("tmp");
        fs::write(&tmp_path, &s)?;
        fs::rename(&tmp_path, &path)?;
    }

    Ok(())
}

fn extract_info_recursive(nodes: &[KdlNode], details: &mut SessionMetadata) {
    for node in nodes {
        match node.name().value() {
            "tab" => details.tabs += 1,
            "cwd" => {
                if let Some(entry) = node.entries().first() {
                    details.cwd = entry.value().to_string().replace('"', "");
                }
            }
            "title" => {
                if let Some(entry) = node.entries().first() {
                    let val = entry.value().to_string().replace('"', "");
                    if val.starts_with('/') || val.starts_with('~') {
                        details.cwd = val;
                    }
                }
            }
            _ => {
                if let Some(cmd_entry) = node
                    .entries()
                    .iter()
                    .find(|e| e.name().map(|n| n.value()) == Some("command"))
                {
                    let cmd = cmd_entry.value().to_string().replace('"', "");
                    let bin = std::path::Path::new(&cmd)
                        .file_name()
                        .and_then(|s| s.to_str())
                        .unwrap_or(&cmd)
                        .to_string();
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

fn extract_info_fallback(content: &str, details: &mut SessionMetadata) {
    if details.tabs == 0 {
        details.tabs = content.matches("tab {").count() + content.matches("tab name=").count();
    }
    if details.cwd == "unknown" {
        let cwd_re = Regex::new(r#"cwd "([^"]+)""#).expect("valid regex");
        if let Some(c) = cwd_re.captures(content).and_then(|c| c.get(1)) {
            details.cwd = c.as_str().to_string();
        }
    }
    let cmd_re = Regex::new(r#"command(?:=| )"([^"]+)""#).expect("valid regex");
    for cap in cmd_re.captures_iter(content) {
        let cmd = cap.get(1).map(|m| m.as_str()).unwrap_or_default();
        let bin = std::path::Path::new(cmd)
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or(cmd)
            .to_string();
        let ignored = ["nu", "bash", "zsh", "fish", "sh", "zellij"];
        if !ignored.contains(&bin.as_str()) && !bin.is_empty() {
            details.commands.push(bin);
        }
    }
}

fn fetch_git_branch(cwd: &str) -> Option<String> {
    let expanded = scanner::resolve_path(cwd);
    if !expanded.exists() {
        return None;
    }
    Command::new("git")
        .args([
            "-C",
            expanded.to_string_lossy().as_ref(),
            "rev-parse",
            "--abbrev-ref",
            "HEAD",
        ])
        .output()
        .ok()
        .filter(|out| out.status.success())
        .map(|out| String::from_utf8_lossy(&out.stdout).trim().to_string())
}
