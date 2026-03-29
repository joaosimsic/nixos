use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

pub fn run() -> Result<()> {
    let home = std::env::var("HOME").context("HOME not set")?;
    let dotfiles = PathBuf::from(&home).join(".config/amber");

    println!("Scanning for configuration folders...");
    visit_dirs(&dotfiles, &home)?;

    Ok(())
}

fn visit_dirs(dir: &Path, home: &str) -> Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                if path.file_name().is_some_and(|name| name == "config") {
                    sync_folder(&path, home)?;
                } else {
                    visit_dirs(&path, home)?;
                }
            }
        }
    }
    Ok(())
}

fn sync_folder(src_path: &Path, home: &str) -> Result<()> {
    let target_name = src_path
        .parent()
        .and_then(|p| p.file_name())
        .and_then(|n| n.to_str())
        .context("Could not determine target name for config")?;

    let dest_name = match target_name {
        "git" => "lazygit",
        "hyprland" => "hypr",
        _ => target_name,
    };

    let dest_path = PathBuf::from(home).join(".config").join(dest_name);

    if dest_path.exists() || dest_path.is_symlink() {
        if dest_path.is_dir() && !dest_path.is_symlink() {
            fs::remove_dir_all(&dest_path)?;
        } else {
            fs::remove_file(&dest_path)?;
        }
    }

    #[cfg(unix)]
    std::os::unix::fs::symlink(src_path, &dest_path)
        .with_context(|| format!("Failed to link {:?} to {:?}", src_path, dest_path))?;

    println!("  synced: {} -> ~/.config/{}", target_name, dest_name);
    Ok(())
}
