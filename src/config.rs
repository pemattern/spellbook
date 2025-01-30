use std::{env, fs};

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct Config {
    pub input: InputConfig,
    pub counter: CounterConfig,
    pub divider: DividerConfig,
    pub application_list: ApplicationListConfig,
}

impl Config {
    pub const PATH: &str = "/Dev/launcher/src/launcher.toml";

    pub fn load() -> Self {
        let path = Self::get_path();
        let toml = fs::read_to_string(&path).unwrap();
        let config = toml::from_str::<Self>(&toml).unwrap();
        config
    }

    pub fn get_path() -> String {
        let home = env::var("HOME").unwrap();
        format!("{}{}", home, Self::PATH)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            input: InputConfig {
                placeholder: String::from("type to filter applications"),
            },
            counter: CounterConfig { display: true },
            divider: DividerConfig { character: '─' },
            application_list: ApplicationListConfig {
                display_icons: true,
            },
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct InputConfig {
    pub placeholder: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct CounterConfig {
    pub display: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DividerConfig {
    pub character: char,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApplicationListConfig {
    pub display_icons: bool,
}
