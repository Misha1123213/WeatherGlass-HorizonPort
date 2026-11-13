// Weather Glass for Horizon - a port of DuffoPuffo's WeatherGlass mod.
// Original: WeatherGlass (NeoForge 26.3). The original draws animated
// weather films on glass block faces; this port shows the same weather
// as a HUD film while you look at glass, using only verified APIs.

use std::sync::atomic::{AtomicU64, Ordering};

pub const ID: &str = "weather_glass";
pub const VERSION: &str = "0.1.0";

#[derive(Clone, Copy, PartialEq)]
pub enum Strength {
    Low,
    Normal,
    High,
}

impl Strength {
    pub fn cycle(self) -> Self {
        match self {
            Self::Low => Self::Normal,
            Self::Normal => Self::High,
            Self::High => Self::Low,
        }
    }
    pub fn label(self) -> &'static str {
        match self {
            Self::Low => "Low",
            Self::Normal => "Normal",
            Self::High => "High",
        }
    }
    pub fn mul(self) -> f32 {
        match self {
            Self::Low => 0.6,
            Self::Normal => 1.0,
            Self::High => 1.5,
        }
    }
}

pub struct Options {
    pub rain: bool,
    pub frost: bool,
    pub dust: bool,
    pub breath: bool,
    pub strength: Strength,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            rain: true,
            frost: true,
            dust: true,
            breath: true,
            strength: Strength::Normal,
        }
    }
}

pub const OPTION_KEYS: [(&str, &str); 5] = [
    ("rain", "Rain on glass"),
    ("frost", "Snow & frost"),
    ("dust", "Dust storms"),
    ("breath", "Breath fog"),
    ("strength", "Strength"),
];

pub fn option(key: &str, value: Option<&String>) -> bool {
    let default = Options::default();
    let default = match key {
        "rain" => default.rain,
        "frost" => default.frost,
        "dust" => default.dust,
        "breath" => default.breath,
        _ => true,
    };
    value.map(|v| v == "true").unwrap_or(default)
}

pub fn strength_from(value: Option<&String>) -> Strength {
    match value.map(String::as_str) {
        Some("Low") => Strength::Low,
        Some("High") => Strength::High,
        _ => Strength::Normal,
    }
}

pub fn options_from(get: &dyn Fn(&str) -> Option<String>) -> Options {
    Options {
        rain: option("rain", get("rain").as_ref()),
        frost: option("frost", get("frost").as_ref()),
        dust: option("dust", get("dust").as_ref()),
        breath: option("breath", get("breath").as_ref()),
        strength: strength_from(get("strength").as_ref()),
    }
}

// --- deterministic animation noise -----------------------------------------

static TICK: AtomicU64 = AtomicU64::new(0);

pub fn next_tick() -> u64 {
    TICK.fetch_add(1, Ordering::Relaxed)
}

/// Stable hash in [0, 1) from a frame counter and an index.
pub fn hash01(tick: u64, i: u32) -> f32 {
    let mut h = tick
        .wrapping_mul(0x9E37_79B9_7F4A_7C15)
        .wrapping_add((i as u64).wrapping_mul(0xBF58_476D_1CE4_E5B9));
    h ^= h >> 29;
    h = h.wrapping_mul(0xBF58_476D_1CE4_E5B9);
    h ^= h >> 32;
    (h >> 40) as f32 / 16_777_216.0
}

// --- world facts --------------------------------------------------------------

/// Block name id, e.g. "minecraft:glass".
pub fn local_name(name: &str) -> &str {
    name.rsplit(':').next().unwrap_or(name)
}

pub fn is_air(name: &str) -> bool {
    local_name(name) == "air"
}

/// Glass blocks, panes, stained, tinted, and other mods' glass.
pub fn is_glass(name: &str) -> bool {
    local_name(name).contains("glass")
}

pub fn is_cold_biome(name: &str) -> bool {
    let n = local_name(name);
    n.contains("snowy")
        || n.contains("frozen")
        || n.contains("ice_spikes")
        || n.contains("jagged_peaks")
        || n.contains("snowy_slopes")
        || n == "grove"
        || n == "deep_frozen_ocean"
}

pub fn is_dry_biome(name: &str) -> bool {
    let n = local_name(name);
    n.contains("desert") || n.contains("badlands") || n.contains("savanna")
}