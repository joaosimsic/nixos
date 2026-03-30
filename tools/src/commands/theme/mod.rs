mod palette;
mod palette_export;
#[cfg(target_os = "linux")]
mod reload;
mod templates;

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

    palette_export::write_palette_json(&dotfiles, &palette)?;
    templates::generate_all(&dotfiles)?;

    #[cfg(target_os = "linux")]
    reload::reload_all(&palette, &dotfiles);

    println!("Done.");
    Ok(())
}

fn dotfiles_dir() -> Result<PathBuf> {
    crate::amber_dir::amber_dir()
}

fn read_theme(path: &PathBuf) -> Result<String> {
    if !path.exists() {
        return Ok("ff6600".to_string());
    }
    let content = std::fs::read_to_string(path).context("reading theme file")?;
    for line in content.lines() {
        if let Some(rest) = line.strip_prefix("base") {
            let hex = rest
                .trim()
                .trim_start_matches('=')
                .trim()
                .trim_start_matches('#');
            return Ok(hex.to_string());
        }
    }
    Ok("ff6600".to_string())
}
