use super::palette::Palette;
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn generate_all(palette: &Palette, dotfiles: &Path) -> Result<()> {
    let vars = palette.as_map();

    visit_dirs(dotfiles, &vars)?;

    Ok(())
}

fn visit_dirs(dir: &Path, vars: &HashMap<String, String>) -> Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, vars)?;
            } else if let Some(ext) = path.extension() {
                if ext == "template" {
                    render_template(&path, vars)?;
                }
            }
        }
    }
    Ok(())
}

fn render_template(template_path: &Path, vars: &HashMap<String, String>) -> Result<()> {
    let content = fs::read_to_string(template_path)
        .with_context(|| format!("failed to read template: {:?}", template_path))?;

    let rendered = substitute(&content, vars);

    let output_path = template_path.with_extension("");

    fs::write(&output_path, rendered)
        .with_context(|| format!("failed to write config: {:?}", output_path))?;

    println!(
        "  rendered: {:?}",
        output_path.file_name().unwrap_or_default()
    );
    Ok(())
}

fn substitute(template: &str, vars: &HashMap<String, String>) -> String {
    let mut out = template.to_string();
    for (key, value) in vars {
        out = out.replace(&format!("{{{{{}}}}}", key), value);
    }
    out
}
