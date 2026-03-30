use super::palette::Palette;
use std::process::{Command, Stdio};

pub fn reload_all(palette: &Palette) {
    reload_hyprland(palette);
    reload_waybar();
    reload_mako();
    restart_ghostty();
    reload_zellij();
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

    let _ = Command::new("hyprctl")
        .arg("reload")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

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

    clear_zellij_cache(&home);

    if std::env::var("ZELLIJ").is_err() {
        let _ = Command::new("pkill")
            .arg("zellij")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
        println!("  zellij sessions cleared");
    } else {
        if let Ok(session) = std::env::var("ZELLIJ_SESSION_NAME") {
            let _ = std::fs::write("/tmp/amber-zellij-reattach", &session);
            let _ = Command::new("zellij")
                .args(["action", "quit"])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status();
        }
        println!("  zellij restarting");
    }
}

fn clear_zellij_cache(home: &str) {
    let cache_dir = std::path::Path::new(home).join(".cache/zellij");
    if !cache_dir.exists() {
        return;
    }

    let permissions_backup = std::fs::read(cache_dir.join("permissions.kdl")).ok();

    let _ = std::fs::remove_dir_all(&cache_dir);
    let _ = std::fs::create_dir_all(&cache_dir);

    if let Some(contents) = permissions_backup {
        let _ = std::fs::write(cache_dir.join("permissions.kdl"), contents);
    }
}

fn restart_ghostty() {
    let _ = Command::new("sh")
        .arg("-c")
        .arg("pkill -USR2 ghostty")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    println!("  ghostty reloaded");
}
