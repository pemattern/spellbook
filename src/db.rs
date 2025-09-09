use std::{env::home_dir, io::Write};

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Db {
    pub entries: Vec<DbEntry>,
}

impl Db {
    const PATH: &str = "/.config/spellbook/";
    const FILENAME: &str = "spells.toml";

    pub fn load() -> Self {
        let path = Self::get_full_path();
        let Ok(toml) = std::fs::read_to_string(&path) else {
            return Self::default();
        };
        let Ok(db) = toml::from_str::<Self>(&toml) else {
            return Self::default();
        };
        db
    }

    pub fn save_to_disk(entries: Vec<DbEntry>) {
        let Ok(toml) = toml::to_string_pretty(&Self { entries }) else {
            return;
        };
        std::fs::create_dir_all(Self::get_path()).unwrap();
        let path = Self::get_full_path();
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(path)
            .unwrap();

        file.write_all(toml.as_bytes()).unwrap();
        file.flush().unwrap();
    }
    fn get_path() -> String {
        let home = home_dir().unwrap();
        format!("{}{}", home.display(), Self::PATH)
    }

    fn get_full_path() -> String {
        format!("{}{}", Self::get_path(), Self::FILENAME)
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct DbEntry {
    pub name: String,
    pub launch_count: usize,
    pub blacklisted: bool,
}

impl DbEntry {
    pub fn new<S: Into<String>>(name: S) -> Self {
        let name = name.into();
        Self {
            name,
            ..Default::default()
        }
    }
}
