use super::palette::Palette;
use std::process::{Command, Stdio};

pub fn reload_all(palette: &Palette) {
    reload_hyprland(palette);
    reload_waybar();
    reload_mako();
    reload_zellij();
    restart_ghostty();
}

fn reload_hyprland(p: &Palette) {
    let args = [
        ("general:col.active_border", format!("rgba({}ff)", p.base)),
        ("general:col.inactive_border", format!("rgba({}ff)", p.dim)),
        ("misc:background_color", format!("rgba({}ff)", p.black)),
    ];
    for (key, value) in &args {
        let _ = Command::new("hyprctl")
            .args(["keyword", key, value])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }
    println!("  hyprland reloaded");
}

fn reload_waybar() {
    let running = Command::new("pgrep")
        .arg("waybar")
        .stdout(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    if running {
        let _ = Command::new("pkill")
            .arg("waybar")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
        let _ = Command::new("waybar")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn();
        println!("  waybar restarted");
    }
}

fn reload_mako() {
    let available = Command::new("which")
        .arg("makoctl")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    if available {
        let _ = Command::new("makoctl")
            .arg("reload")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
        println!("  mako reloaded");
    }
}

fn reload_zellij() {
    let home = match std::env::var("HOME") {
        Ok(h) => h,
        Err(_) => return,
    };

    let cache_session_dirs: &[&str] = &[
        "contract_version_1/session_info",
        "0.44.0/session_info",
        "0.43.1/session_info",
    ];

    for dir in cache_session_dirs {
        let path = std::path::Path::new(&home).join(".cache/zellij").join(dir);
        if path.exists() {
            let _ = std::fs::remove_dir_all(&path);
        }
    }

    let _ = Command::new("pkill")
        .arg("-x")
        .arg("zellij")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    println!("  zellij sessions cleared");
}

fn restart_ghostty() {
    let status = Command::new("sh")
        .arg("-c")
        .arg("pkill -USR2 ghostty") 
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    if status.map(|s| s.success()).unwrap_or(false) {
        println!("  ghostty reloaded");
    } else {
        println!("  [error] ghostty reload command failed");
    }
}
