mod palette;
mod templates;
#[cfg(target_os = "linux")]
mod reload;

use anyhow::{Context, Result};
use palette::Palette;
use std::path::PathBuf;

pub fn run(color: Option<String>, dry_run: bool) -> Result<()> {
    let dotfiles = dotfiles_dir()?;
    let theme_file = dotfiles.join("theme");

    let hex = match color {
        Some(c) => {
            let hex = c.trim_start_matches('#').to_string();
            if !dry_run {
                std::fs::write(&theme_file, format!("base = #{}\n", hex))
                    .context("writing theme file")?;
            }
            hex
        }
        None => read_theme(&theme_file)?,
    };

    let palette = Palette::from_hex(&hex)?;

    println!("Theme: #{}", palette.base);
    println!(
        "  bright=#{} dim=#{} surface=#{} black=#{}",
        palette.bright, palette.dim, palette.surface, palette.black
    );
    println!(
        "  red=#{} green=#{} yellow=#{} blue=#{}",
        palette.red, palette.green, palette.yellow, palette.blue
    );

    if dry_run {
        return Ok(());
    }

    templates::generate_all(&palette, &dotfiles)?;

    #[cfg(target_os = "linux")]
    reload::reload_all(&palette);

    println!("Done.");
    Ok(())
}

fn dotfiles_dir() -> Result<PathBuf> {
    if let Ok(dir) = std::env::var("AMBER_DIR") {
        return Ok(PathBuf::from(dir));
    }
    let home = std::env::var("HOME").context("HOME not set")?;
    Ok(PathBuf::from(home).join(".config/amber"))
}

fn read_theme(path: &PathBuf) -> Result<String> {
    if !path.exists() {
        return Ok("ff6600".to_string());
    }
    let content = std::fs::read_to_string(path).context("reading theme file")?;
    for line in content.lines() {
        if let Some(rest) = line.strip_prefix("base") {
            let hex = rest.trim().trim_start_matches('=').trim().trim_start_matches('#');
            return Ok(hex.to_string());
        }
    }
    Ok("ff6600".to_string())
}
