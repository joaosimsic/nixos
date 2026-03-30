use anyhow::{Context, Result};
use std::path::PathBuf;

pub fn amber_dir() -> Result<PathBuf> {
    if let Ok(dir) = std::env::var("AMBER_DIR") {
        return Ok(PathBuf::from(dir));
    }
    let home = std::env::var("HOME").context("HOME not set; set HOME or export AMBER_DIR")?;
    Ok(PathBuf::from(home).join(".config/amber"))
}

pub fn palette_json_path() -> Result<PathBuf> {
    Ok(amber_dir()?.join("palette.json"))
}
