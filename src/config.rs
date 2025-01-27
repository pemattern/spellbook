use std::{env, fs};

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub placeholder: Option<String>,
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
