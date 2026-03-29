use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

#[derive(Debug)]
pub struct Monitor {
    pub name: String,
    pub resolution: String,
    pub refresh_rate: u32,
}

#[derive(Debug)]
pub struct MonitorConfig {
    pub primary: Monitor,
    pub secondary: Monitor,
}

impl MonitorConfig {
    pub fn load(amber_root: &Path) -> Result<Self> {
        let hostname = hostname::get()
            .context("Failed to get hostname")?
            .to_string_lossy()
            .to_string();

        let monitors_path = amber_root
            .join("hosts")
            .join(&hostname)
            .join("monitors.nix");

        let content = fs::read_to_string(&monitors_path)
            .with_context(|| format!("Failed to read monitors.nix for host '{}'", hostname))?;

        Self::parse(&content)
    }

    fn parse(content: &str) -> Result<Self> {
        Ok(MonitorConfig {
            primary: Self::parse_monitor(content, "primary")?,
            secondary: Self::parse_monitor(content, "secondary")?,
        })
    }

    fn parse_monitor(content: &str, monitor_type: &str) -> Result<Monitor> {
        let section_start = content
            .find(&format!("{} = {{", monitor_type))
            .with_context(|| format!("Could not find {} monitor section", monitor_type))?;

        let section = &content[section_start..];
        let section_end = section.find('}').unwrap_or(section.len());
        let section = &section[..section_end];

        let name = Self::extract_string_field(section, "name")
            .with_context(|| format!("Could not find name for {} monitor", monitor_type))?;

        let resolution = Self::extract_string_field(section, "resolution")
            .with_context(|| format!("Could not find resolution for {} monitor", monitor_type))?;

        let refresh_rate = Self::extract_int_field(section, "refreshRate")
            .with_context(|| format!("Could not find refreshRate for {} monitor", monitor_type))?;

        Ok(Monitor {
            name,
            resolution,
            refresh_rate,
        })
    }

    fn extract_string_field(section: &str, field: &str) -> Option<String> {
        let pattern = format!("{} = \"", field);
        let start = section.find(&pattern)? + pattern.len();
        let rest = &section[start..];
        let end = rest.find('"')?;
        Some(rest[..end].to_string())
    }

    fn extract_int_field(section: &str, field: &str) -> Option<u32> {
        let pattern = format!("{} = ", field);
        let start = section.find(&pattern)? + pattern.len();
        let rest = &section[start..];
        let end = rest.find(|c: char| !c.is_ascii_digit())?;
        rest[..end].parse().ok()
    }
}

pub fn generate_hyprland_monitors_conf(config: &MonitorConfig) -> String {
    let p = &config.primary;
    let s = &config.secondary;

    format!(
        r#"monitor = {}, {}@{}, 0x0, 1
monitor = {}, {}@{}, 1920x0, 1

workspace = 1, monitor:{}
workspace = 2, monitor:{}
workspace = 3, monitor:{}
workspace = 4, monitor:{}
workspace = 5, monitor:{}
workspace = 6, monitor:{}
workspace = 7, monitor:{}
workspace = 8, monitor:{}
workspace = 9, monitor:{}
workspace = 10, monitor:{}
"#,
        p.name,
        p.resolution,
        p.refresh_rate,
        s.name,
        s.resolution,
        s.refresh_rate,
        p.name,
        p.name,
        p.name,
        p.name,
        p.name,
        s.name,
        s.name,
        s.name,
        s.name,
        s.name,
    )
}
