use std::{env, fs};

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Config {
    pub input: InputConfig,
    pub counter: CounterConfig,
    pub margin: MarginConfig,
    pub application_list: ApplicationListConfig,
    pub scrollbar: ScrollbarConfig,
    pub info: InfoConfig,
    pub color_mode: ColorMode,
}

impl Config {
    const PATH: &str = "/.config/spellbook/";
    const FILENAME: &str = "spellbook.toml";

    pub fn load() -> Self {
        let path = Self::get_full_path();
        let Ok(toml) = fs::read_to_string(&path) else {
            return Self::default();
        };
        toml::from_str::<Self>(&toml).unwrap_or_else(|error| {
            ratatui::restore();
            panic!("{}", error);
        })
    }

    fn get_path() -> String {
        let home = env::var("HOME").unwrap();
        format!("{}{}", home, Self::PATH)
    }

    pub fn get_full_path() -> String {
        format!("{}{}", Self::get_path(), Self::FILENAME)
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct InputConfig {
    pub icon: String,
    pub placeholder: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct CounterConfig {
    pub enable: bool,
    pub bold: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct MarginConfig {
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ApplicationListConfig {
    pub display_icons: bool,
    pub order: ApplicationListOrder,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum ApplicationListOrder {
    Alphabetical,
    #[default]
    MostUsed,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ScrollbarConfig {
    pub enable: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct InfoConfig {
    pub enable: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum ColorMode {
    Light,
    #[default]
    Dark,
}
