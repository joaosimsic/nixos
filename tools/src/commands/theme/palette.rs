use anyhow::{bail, Result};

pub struct Palette {
    pub base: String,
    pub bright: String,
    pub dim: String,
    pub surface: String,
    pub bg: String,
    pub black: String,
    pub comment: String,
    pub red: String,
    pub red_bright: String,
    pub green: String,
    pub green_bright: String,
    pub yellow: String,
    pub yellow_bright: String,
    pub blue: String,
    pub blue_bright: String,
    pub magenta: String,
    pub magenta_bright: String,
    pub cyan: String,
    pub cyan_bright: String,
    pub error: String,
    pub error_bright: String,
}

impl Palette {
    pub fn from_hex(hex: &str) -> Result<Self> {
        let hex = hex.trim_start_matches('#');
        if hex.len() != 6 {
            bail!("invalid hex color: #{}", hex);
        }
        let r = u8::from_str_radix(&hex[0..2], 16)? as f64 / 255.0;
        let g = u8::from_str_radix(&hex[2..4], 16)? as f64 / 255.0;
        let b = u8::from_str_radix(&hex[4..6], 16)? as f64 / 255.0;

        let (h, l, s) = rgb_to_hls(r, g, b);

        let h_red = (h - 0.05).rem_euclid(1.0);
        let h_yellow = (h + 0.10).rem_euclid(1.0);
        let h_green = (h + 0.20).rem_euclid(1.0);
        let h_cyan = (h + 0.15).rem_euclid(1.0);
        let h_blue = (h - 0.15).rem_euclid(1.0);
        let h_magenta = (h - 0.10).rem_euclid(1.0);

        let red = hls_hex(h_red, l * 0.65, s * 0.75);
        let red_bright = hls_hex(h_red, (l * 0.85).min(0.70), s * 0.75);

        Ok(Self {
            base: hex.to_lowercase(),
            bright: hls_hex(h, (l * 1.35).min(0.92), s),
            dim: hls_hex(h, l * 0.45, s),
            surface: hls_hex(h, l * 0.08, s),
            bg: hls_hex(h, l * 0.10, s),
            black: hls_hex(h, l * 0.04, s),
            comment: hls_hex(h, l * 0.55, s * 0.6),
            error: red.clone(),
            error_bright: red_bright.clone(),
            red,
            red_bright,
            green: hls_hex(h_green, l * 0.70, s * 0.70),
            green_bright: hls_hex(h_green, (l * 0.90).min(0.75), s * 0.70),
            yellow: hls_hex(h_yellow, l * 0.75, s * 0.70),
            yellow_bright: hls_hex(h_yellow, (l * 0.95).min(0.80), s * 0.70),
            blue: hls_hex(h_blue, l * 0.60, s * 0.65),
            blue_bright: hls_hex(h_blue, (l * 0.80).min(0.70), s * 0.65),
            magenta: hls_hex(h_magenta, l * 0.65, s * 0.65),
            magenta_bright: hls_hex(h_magenta, (l * 0.85).min(0.75), s * 0.65),
            cyan: hls_hex(h_cyan, l * 0.60, s * 0.65),
            cyan_bright: hls_hex(h_cyan, (l * 0.80).min(0.70), s * 0.65),
        })
    }

    pub fn rgb_dec(hex: &str) -> String {
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
        format!("{} {} {}", r, g, b)
    }
}

fn hls_hex(h: f64, l: f64, s: f64) -> String {
    let (r, g, b) = hls_to_rgb(h, l.clamp(0.0, 1.0), s.clamp(0.0, 1.0));
    let to_u8 = |v: f64| (v.clamp(0.0, 1.0) * 255.0).round() as u8;
    format!("{:02x}{:02x}{:02x}", to_u8(r), to_u8(g), to_u8(b))
}

fn rgb_to_hls(r: f64, g: f64, b: f64) -> (f64, f64, f64) {
    let maxc = r.max(g).max(b);
    let minc = r.min(g).min(b);
    let l = (minc + maxc) / 2.0;
    if (maxc - minc).abs() < f64::EPSILON {
        return (0.0, l, 0.0);
    }
    let s = if l <= 0.5 {
        (maxc - minc) / (maxc + minc)
    } else {
        (maxc - minc) / (2.0 - maxc - minc)
    };
    let rc = (maxc - r) / (maxc - minc);
    let gc = (maxc - g) / (maxc - minc);
    let bc = (maxc - b) / (maxc - minc);
    let h = if (r - maxc).abs() < f64::EPSILON {
        bc - gc
    } else if (g - maxc).abs() < f64::EPSILON {
        2.0 + rc - bc
    } else {
        4.0 + gc - rc
    };
    ((h / 6.0).rem_euclid(1.0), l, s)
}

fn hls_to_rgb(h: f64, l: f64, s: f64) -> (f64, f64, f64) {
    if s.abs() < f64::EPSILON {
        return (l, l, l);
    }
    let m2 = if l <= 0.5 { l * (1.0 + s) } else { l + s - l * s };
    let m1 = 2.0 * l - m2;
    (
        hue_to_rgb(m1, m2, h + 1.0 / 3.0),
        hue_to_rgb(m1, m2, h),
        hue_to_rgb(m1, m2, h - 1.0 / 3.0),
    )
}

fn hue_to_rgb(m1: f64, m2: f64, hue: f64) -> f64 {
    let hue = hue.rem_euclid(1.0);
    if hue < 1.0 / 6.0 {
        m1 + (m2 - m1) * hue * 6.0
    } else if hue < 0.5 {
        m2
    } else if hue < 2.0 / 3.0 {
        m1 + (m2 - m1) * (2.0 / 3.0 - hue) * 6.0
    } else {
        m1
    }
}