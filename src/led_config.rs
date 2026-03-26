use serde::Deserialize;
use std::fs;

#[derive(Debug, Clone, Deserialize, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum LedMode {
    /// Don't send any LED command — device keeps its current state
    #[default]
    None,
    /// Static color(s)
    Static,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LedConfig {
    #[serde(default)]
    pub mode: LedMode,
    /// LED brightness 0-100, only used in static mode
    #[serde(default = "default_brightness")]
    pub brightness: u8,
    /// Single RGB color for all knob LEDs (used when `colors` is not set)
    pub color: Option<[u8; 3]>,
    /// Per-LED RGB colors, overrides `color` when set
    pub colors: Option<Vec<[u8; 3]>>,
}

fn default_brightness() -> u8 {
    100
}

impl LedConfig {
    /// Returns the RGB bytes to send: one entry per LED, 4 total.
    /// `colors` takes priority over `color`; missing entries fall back to white.
    pub fn resolved_colors(&self) -> [[u8; 3]; 4] {
        let fallback = self.color.unwrap_or([255, 255, 255]);
        let mut out = [fallback; 4];
        if let Some(colors) = &self.colors {
            for (i, c) in colors.iter().take(4).enumerate() {
                out[i] = *c;
            }
        }
        out
    }
}

impl Default for LedConfig {
    fn default() -> Self {
        Self {
            mode: LedMode::None,
            brightness: default_brightness(),
            color: None,
            colors: None,
        }
    }
}

pub fn load() -> LedConfig {
    let path = dirs_config_path();

    match path.and_then(|p| fs::read_to_string(p).ok()) {
        Some(contents) => toml::from_str(&contents).unwrap_or_else(|e| {
            log::warn!("Failed to parse LED config, using defaults: {e}");
            LedConfig::default()
        }),
        None => LedConfig::default(),
    }
}

fn dirs_config_path() -> Option<std::path::PathBuf> {
    Some(dirs::config_dir()?.join("opendeck-akp05").join("leds.toml"))
}
