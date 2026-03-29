use super::palette::Palette;
use anyhow::{Context, Result};
use std::path::Path;

pub fn generate_all(palette: &Palette, dotfiles: &Path) -> Result<()> {
    let vars = build_vars(palette);

    let mappings: &[(&str, &str)] = &[
        (
            "domains/wm/hyprland/config/colors.conf.template",
            "domains/wm/hyprland/config/colors.conf",
        ),
        (
            "domains/bar/waybar/config/colors.css.template",
            "domains/bar/waybar/config/colors.css",
        ),
        (
            "domains/terminal/ghostty/config/colors.conf.template",
            "domains/terminal/ghostty/config/colors.conf",
        ),
        (
            "domains/notifications/mako/config/config.template",
            "domains/notifications/mako/config/config",
        ),
        (
            "domains/launcher/rofi/config/theme.rasi.template",
            "domains/launcher/rofi/config/theme.rasi",
        ),
        (
            "domains/shell/nushell/config/scripts/grave.nu.template",
            "domains/shell/nushell/config/scripts/grave.nu",
        ),
        (
            "domains/shell/nushell/config/colors.nu.template",
            "domains/shell/nushell/config/colors.nu",
        ),
        (
            "domains/shell/nushell/config/starship.toml.template",
            "domains/shell/nushell/config/starship.toml",
        ),
        (
            "domains/editor/nvim/config/lua/config/theme.lua.template",
            "domains/editor/nvim/config/lua/config/theme.lua",
        ),
        (
            "capabilities/git/config/config.yml.template",
            "capabilities/git/config/config.yml",
        ),
        (
            "domains/terminal/zellij/config/config.kdl.template",
            "domains/terminal/zellij/config/config.kdl",
        ),
        (
            "domains/terminal/zellij/config/layouts/default.kdl.template",
            "domains/terminal/zellij/config/layouts/default.kdl",
        ),
    ];

    for (template_rel, output_rel) in mappings {
        let template_path = dotfiles.join(template_rel);
        if !template_path.exists() {
            continue;
        }
        let content = std::fs::read_to_string(&template_path)
            .with_context(|| format!("reading {}", template_path.display()))?;
        let rendered = substitute(&content, &vars);
        let output_path = dotfiles.join(output_rel);
        std::fs::write(&output_path, rendered)
            .with_context(|| format!("writing {}", output_path.display()))?;
    }

    Ok(())
}

fn substitute(template: &str, vars: &[(&str, String)]) -> String {
    let mut out = template.to_string();
    for (key, value) in vars {
        out = out.replace(&format!("{{{{{}}}}}", key), value);
    }
    out
}

fn build_vars(p: &Palette) -> Vec<(&'static str, String)> {
    vec![
        ("BASE", p.base.clone()),
        ("BRIGHT", p.bright.clone()),
        ("DIM", p.dim.clone()),
        ("SURFACE", p.surface.clone()),
        ("BG", p.bg.clone()),
        ("BLACK", p.black.clone()),
        ("COMMENT", p.comment.clone()),
        ("RED", p.red.clone()),
        ("RED_BRIGHT", p.red_bright.clone()),
        ("GREEN", p.green.clone()),
        ("GREEN_BRIGHT", p.green_bright.clone()),
        ("YELLOW", p.yellow.clone()),
        ("YELLOW_BRIGHT", p.yellow_bright.clone()),
        ("BLUE", p.blue.clone()),
        ("BLUE_BRIGHT", p.blue_bright.clone()),
        ("MAGENTA", p.magenta.clone()),
        ("MAGENTA_BRIGHT", p.magenta_bright.clone()),
        ("CYAN", p.cyan.clone()),
        ("CYAN_BRIGHT", p.cyan_bright.clone()),
        ("ERROR", p.error.clone()),
        ("ERROR_BRIGHT", p.error_bright.clone()),
        ("BASE_RGB", Palette::rgb_dec(&p.base)),
        ("BRIGHT_RGB", Palette::rgb_dec(&p.bright)),
        ("DIM_RGB", Palette::rgb_dec(&p.dim)),
        ("BLACK_RGB", Palette::rgb_dec(&p.black)),
        ("RED_RGB", Palette::rgb_dec(&p.red)),
        ("P0", p.black.clone()),
        ("P1", p.red.clone()),
        ("P2", p.green.clone()),
        ("P3", p.yellow.clone()),
        ("P4", p.blue.clone()),
        ("P5", p.magenta.clone()),
        ("P6", p.cyan.clone()),
        ("P7", p.base.clone()),
        ("P8", p.dim.clone()),
        ("P9", p.red_bright.clone()),
        ("P10", p.green_bright.clone()),
        ("P11", p.yellow_bright.clone()),
        ("P12", p.blue_bright.clone()),
        ("P13", p.magenta_bright.clone()),
        ("P14", p.cyan_bright.clone()),
        ("P15", p.bright.clone()),
    ]
}
