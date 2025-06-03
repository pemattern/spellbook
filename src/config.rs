use std::{env, fs};

use serde::Deserialize;

#[derive(Debug, Default, Deserialize)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub input: InputConfig,
    pub counter: CounterConfig,
    pub border: BorderConfig,
    pub application_list: ApplicationListConfig,
    pub info: InfoConfig,
    pub color_mode: ColorMode,
}

impl Config {
    const PATH: &str = "/Dev/spellbook/spellbook.toml";

    pub fn load() -> Self {
        let path = Self::get_path();
        let toml = fs::read_to_string(&path).unwrap();
        toml::from_str::<Self>(&toml).unwrap()
    }

    pub fn get_path() -> String {
        let home = env::var("HOME").unwrap();
        format!("{}{}", home, Self::PATH)
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct InputConfig {
    pub placeholder: String,
}

impl Default for InputConfig {
    fn default() -> Self {
        Self {
            placeholder: String::from("Search Applications"),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct CounterConfig {
    pub enable: bool,
    pub bold: bool,
}

impl Default for CounterConfig {
    fn default() -> Self {
        Self {
            enable: true,
            bold: false,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct BorderConfig {
    pub margin: MarginConfig,
    pub enable_border: bool,
    pub divider_character: char,
}

impl Default for BorderConfig {
    fn default() -> Self {
        Self {
            margin: MarginConfig::default(),
            enable_border: true,
            divider_character: '─',
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct MarginConfig {
    pub x: u16,
    pub y: u16,
}

impl Default for MarginConfig {
    fn default() -> Self {
        Self { x: 3, y: 1 }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct ApplicationListConfig {
    pub display_icons: bool,
}

impl Default for ApplicationListConfig {
    fn default() -> Self {
        Self {
            display_icons: true,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct InfoConfig {
    pub enable: bool,
}

impl Default for InfoConfig {
    fn default() -> Self {
        Self { enable: true }
    }
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum ColorMode {
    Light,
    #[default]
    Dark,
}
