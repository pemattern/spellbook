use serde::{Deserialize, Serialize};

pub struct Db;

impl Db {
    const PATH: &str = "/Dev/spellbook/spells.toml";

    pub fn load() -> Vec<DbEntry> {
        let path = Self::get_path();
        let toml = std::fs::read_to_string(&path).unwrap();
        toml::from_str::<Vec<DbEntry>>(&toml).unwrap()
    }

    pub fn get_path() -> String {
        let home = std::env::var("HOME").unwrap();
        format!("{}{}", home, Self::PATH)
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields, rename = "entry")]
pub struct DbEntry {
    name: String,
    launch_count: usize,
    blacklisted: bool,
}
