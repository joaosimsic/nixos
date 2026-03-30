use super::palette::Palette;
use anyhow::{Context, Result};
use serde_json::{Map, Value};
use std::fs;
use std::path::Path;

/// Single on-disk color table for `*.template` injection and for tools (e.g. `amber grave`).
pub fn write_palette_json(dotfiles: &Path, palette: &Palette) -> Result<()> {
    let path = dotfiles.join("palette.json");
    let map = palette.as_map();
    let mut obj = Map::new();
    for (k, v) in map {
        obj.insert(k, Value::String(v));
    }
    let json = serde_json::to_string_pretty(&Value::Object(obj)).context("serialize palette")?;
    fs::write(&path, format!("{json}\n")).with_context(|| format!("write {}", path.display()))?;
    println!("  wrote {}", path.display());
    Ok(())
}
