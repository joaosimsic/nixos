use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

use super::monitors::{generate_hyprland_monitors_conf, MonitorConfig};

pub fn run() -> Result<()> {
    let home = std::env::var("HOME").context("HOME not set")?;
    let amber_root = PathBuf::from(&home).join(".config/amber");

    let hostname = hostname::get()
        .context("Failed to get hostname")?
        .to_string_lossy()
        .to_string();

    println!("Syncing configurations for host '{}'...", hostname);
    println!();

    let monitor_config = MonitorConfig::load(&amber_root).ok();

    visit_dirs(&amber_root, &home, &monitor_config)?;

    let starship_src = amber_root.join("domains/shell/nushell/config/starship.toml");
    if starship_src.exists() {
        let starship_dest = PathBuf::from(&home).join(".config/starship.toml");
        symlink_force(&starship_src, &starship_dest)?;
        println!(" -> starship.toml");
    }

    println!();
    println!("Done. Configs synced.");

    Ok(())
}

fn visit_dirs(dir: &Path, home: &str, monitor_config: &Option<MonitorConfig>) -> Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                if path.file_name().is_some_and(|name| name == "config") {
                    sync_folder(&path, home, monitor_config)?;
                } else {
                    visit_dirs(&path, home, monitor_config)?;
                }
            }
        }
    }
    Ok(())
}

fn sync_folder(src_path: &Path, home: &str, monitor_config: &Option<MonitorConfig>) -> Result<()> {
    let target_name = src_path
        .parent()
        .and_then(|p| p.file_name())
        .and_then(|n| n.to_str())
        .context("Could not determine target name for config")?;

    let dest_name = match target_name {
        "hyprland" => "hypr",
        "git" => "lazygit",
        _ => target_name,
    };

    let dest_path = PathBuf::from(home).join(".config").join(dest_name);

    match target_name {
        "hyprland" => {
            sync_hyprland(src_path, &dest_path, monitor_config)?;
            return Ok(());
        }
        "waybar" => {
            sync_waybar(src_path, &dest_path, monitor_config)?;
            return Ok(());
        }
        _ => {}
    }

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

    println!(" -> {}", dest_name);
    Ok(())
}

fn sync_hyprland(
    src_path: &Path,
    dest_path: &Path,
    monitor_config: &Option<MonitorConfig>,
) -> Result<()> {
    if dest_path.is_symlink() {
        fs::remove_file(dest_path)?;
    }
    fs::create_dir_all(dest_path)?;

    for entry in fs::read_dir(src_path)? {
        let entry = entry?;
        let path = entry.path();
        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        if file_name == "monitors.conf" || file_name.ends_with(".template") {
            continue;
        }

        let dest_file = dest_path.join(file_name);
        symlink_force(&path, &dest_file)?;
    }

    match monitor_config {
        Some(config) => {
            let monitors_conf = generate_hyprland_monitors_conf(config);
            fs::write(dest_path.join("monitors.conf"), monitors_conf)?;
            println!(" -> hypr (with generated monitors.conf)");
        }
        None => {
            println!(" ! hypr: Could not read monitors config");
        }
    }

    Ok(())
}

fn sync_waybar(
    src_path: &Path,
    dest_path: &Path,
    monitor_config: &Option<MonitorConfig>,
) -> Result<()> {
    if dest_path.is_symlink() {
        fs::remove_file(dest_path)?;
    }
    fs::create_dir_all(dest_path)?;

    let style_src = src_path.join("style.css");
    let colors_src = src_path.join("colors.css");

    if style_src.exists() {
        symlink_force(&style_src, &dest_path.join("style.css"))?;
    }
    if colors_src.exists() {
        symlink_force(&colors_src, &dest_path.join("colors.css"))?;
    }

    let config_src = src_path.join("config");
    if config_src.exists() {
        match monitor_config {
            Some(config) => {
                let content = fs::read_to_string(&config_src)?;
                let generated = generate_waybar_config(&content, config)?;
                fs::write(dest_path.join("config"), generated)?;
                println!(" -> waybar (with generated config)");
            }
            None => {
                symlink_force(&config_src, &dest_path.join("config"))?;
                println!(" ! waybar: Could not read monitors config");
            }
        }
    }

    Ok(())
}

fn generate_waybar_config(content: &str, config: &MonitorConfig) -> Result<String> {
    let mut data: serde_json::Value =
        serde_json::from_str(content).context("Failed to parse waybar config as JSON")?;

    if let Some(arr) = data.as_array_mut() {
        if let Some(bar) = arr.get_mut(0) {
            bar["output"] = serde_json::json!(config.primary.name);
            bar["hyprland/workspaces"]["persistent-workspaces"] =
                serde_json::json!({ &config.primary.name: [1, 2, 3, 4, 5] });
        }

        if let Some(bar) = arr.get_mut(1) {
            bar["output"] = serde_json::json!(config.secondary.name);
            bar["hyprland/workspaces"]["persistent-workspaces"] =
                serde_json::json!({ &config.secondary.name: [6, 7, 8, 9, 10] });
        }
    }

    serde_json::to_string_pretty(&data).context("Failed to serialize waybar config")
}

fn symlink_force(src: &Path, dest: &Path) -> Result<()> {
    if dest.exists() || dest.is_symlink() {
        fs::remove_file(dest)?;
    }

    #[cfg(unix)]
    std::os::unix::fs::symlink(src, dest)
        .with_context(|| format!("Failed to link {:?} to {:?}", src, dest))?;

    Ok(())
}

pub fn status() -> Result<()> {
    let home = std::env::var("HOME").context("HOME not set")?;
    let amber_root = PathBuf::from(&home).join(".config/amber");

    let hostname = hostname::get()
        .context("Failed to get hostname")?
        .to_string_lossy()
        .to_string();

    println!("Amber Config Status (host: {})", hostname);
    println!("=======================================");
    println!();

    visit_dirs_status(&amber_root, &home)?;

    Ok(())
}

fn visit_dirs_status(dir: &Path, home: &str) -> Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                if path.file_name().is_some_and(|name| name == "config") {
                    check_status(&path, home)?;
                } else {
                    visit_dirs_status(&path, home)?;
                }
            }
        }
    }
    Ok(())
}

fn check_status(src_path: &Path, home: &str) -> Result<()> {
    let target_name = src_path
        .parent()
        .and_then(|p| p.file_name())
        .and_then(|n| n.to_str())
        .context("Could not determine target name")?;

    let dest_name = match target_name {
        "hyprland" => "hypr",
        "git" => "lazygit",
        _ => target_name,
    };

    let dest_path = PathBuf::from(home).join(".config").join(dest_name);

    let is_special = target_name == "hyprland" || target_name == "waybar";

    if is_special {
        if dest_path.is_dir() {
            println!(" [ok] {} (mixed: symlinks + generated)", dest_name);
        } else {
            println!(" [--] {} (not synced)", dest_name);
        }
    } else if dest_path.is_symlink() {
        let link_target = fs::read_link(&dest_path)?;
        if link_target == src_path {
            println!(" [ok] {}", dest_name);
        } else {
            println!(" [!!] {} (wrong target)", dest_name);
        }
    } else if dest_path.is_dir() {
        println!(" [!!] {} (directory, not synced)", dest_name);
    } else {
        println!(" [--] {} (not linked)", dest_name);
    }

    Ok(())
}
