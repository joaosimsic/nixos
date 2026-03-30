use anyhow::{Context, Result};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Renders every `*.template` under the amber tree using placeholders `{{KEY}}`.
/// Variables come only from `palette.json` in the amber root (written by `palette_export` first).
pub fn generate_all(dotfiles: &Path) -> Result<()> {
    let vars = load_substitution_map(dotfiles)?;
    visit_dirs(dotfiles, &vars)?;
    Ok(())
}

fn load_substitution_map(dotfiles: &Path) -> Result<HashMap<String, String>> {
    let path = dotfiles.join("palette.json");
    let content = fs::read_to_string(&path).with_context(|| {
        format!(
            "missing {}; run palette export before template injection",
            path.display()
        )
    })?;
    let v: Value = serde_json::from_str(&content)
        .with_context(|| format!("invalid JSON in {}", path.display()))?;
    let obj = v
        .as_object()
        .with_context(|| format!("{} must be a JSON object", path.display()))?;

    let mut vars = HashMap::new();
    for (k, val) in obj {
        if let Some(s) = val.as_str() {
            vars.insert(k.clone(), s.to_string());
        }
    }
    Ok(vars)
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
