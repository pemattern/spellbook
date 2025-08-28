use std::{env, fs, io::Write};

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Config {
    pub input: InputConfig,
    pub counter: CounterConfig,
    pub border: BorderConfig,
    pub application_list: ApplicationListConfig,
    pub info: InfoConfig,
    pub color_mode: ColorMode,
}

impl Config {
    const PATH: &str = "/.config/spellbook/";
    const FILENAME: &str = "spellbook.toml";

    pub fn load() -> Self {
        let path = Self::get_full_path();
        let Ok(toml) = fs::read_to_string(&path) else {
            return Self::save_default_config();
        };
        let Ok(config) = toml::from_str::<Self>(&toml) else {
            return Self::save_default_config();
        };
        config
    }

    fn save_default_config() -> Self {
        let config = Config::default();
        let path = Self::get_full_path();
        let toml = toml::to_string_pretty(&config).unwrap();
        std::fs::create_dir_all(Self::get_path()).unwrap();
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)
            .unwrap();
        file.write_all(toml.as_bytes()).unwrap();
        file.flush().unwrap();
        config
    }

    fn get_path() -> String {
        let home = env::var("HOME").unwrap();
        format!("{}{}", home, Self::PATH)
    }

    pub fn get_full_path() -> String {
        format!("{}{}", Self::get_path(), Self::FILENAME)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct BorderConfig {
    pub margin: MarginConfig,
    pub enable_border: bool,
    pub divider_character: char,
}

impl Default for BorderConfig {
    fn default() -> Self {
        Self {
            margin: MarginConfig::default(),
            enable_border: false,
            divider_character: '─',
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct MarginConfig {
    pub x: u16,
    pub y: u16,
}

impl Default for MarginConfig {
    fn default() -> Self {
        Self { x: 1, y: 0 }
    }
}

#[derive(Debug, Serialize, Deserialize)]
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

impl Default for ApplicationListConfig {
    fn default() -> Self {
        Self {
            display_icons: true,
            order: ApplicationListOrder::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct InfoConfig {
    pub enable: bool,
}

impl Default for InfoConfig {
    fn default() -> Self {
        Self { enable: true }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub enum ColorMode {
    Light,
    #[default]
    Dark,
}
