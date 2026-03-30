use super::palette::Palette;
use std::path::Path;
use std::process::{Command, Stdio};

pub fn reload_all(palette: &Palette, dotfiles: &Path) {
    reload_hyprland(palette);
    reload_waybar();
    reload_mako();
    restart_ghostty();
    reload_foot();
    reload_kitty();
    reload_wezterm();
    reload_dunst();
    reload_swaync();
    reload_eww();
    reload_zellij();
    reload_neovim_theme(dotfiles);
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

fn process_running(name: &str) -> bool {
    Command::new("pgrep")
        .args(["-x", name])
        .stdout(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn command_available(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

fn reload_foot() {
    if !process_running("foot") {
        return;
    }
    let _ = Command::new("pkill")
        .args(["-USR1", "foot"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    println!("  foot reloaded");
}

fn reload_kitty() {
    if !process_running("kitty") {
        return;
    }
    let _ = Command::new("pkill")
        .args(["-USR1", "kitty"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
    println!("  kitty reloaded");
}

fn reload_wezterm() {
    if !command_available("wezterm") {
        return;
    }
    let ok = Command::new("wezterm")
        .args(["cli", "reload"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    if ok {
        println!("  wezterm reloaded");
    }
}

fn reload_dunst() {
    if !command_available("dunstctl") {
        return;
    }
    let ok = Command::new("dunstctl")
        .arg("reload")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    if ok {
        println!("  dunst reloaded");
    }
}

fn reload_swaync() {
    if !command_available("swaync-client") {
        return;
    }
    let ok = Command::new("swaync-client")
        .arg("-R")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    if ok {
        println!("  swaync reloaded");
    }
}

fn reload_eww() {
    if !command_available("eww") {
        return;
    }
    let daemon_up = Command::new("eww")
        .arg("ping")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    if !daemon_up {
        return;
    }
    let ok = Command::new("eww")
        .arg("reload")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    if ok {
        println!("  eww reloaded");
    }
}

fn reload_neovim_theme(dotfiles: &Path) {
    let home = match std::env::var("HOME") {
        Ok(h) => h,
        Err(_) => return,
    };
    let candidates = [
        dotfiles.join("domains/editor/nvim/config/lua/config/theme.lua"),
        Path::new(&home).join(".config/nvim/lua/config/theme.lua"),
    ];
    let Some(theme_lua) = candidates.iter().find(|p| p.exists()) else {
        return;
    };
    let path = theme_lua.to_string_lossy();
    let ok = Command::new("nvr")
        .args(["-c", &format!("luafile {path}")])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false);
    if ok {
        println!("  neovim theme.lua reloaded (nvr)");
    }
}
