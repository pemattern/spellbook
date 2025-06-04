use std::io::Write;

use serde::{Deserialize, Serialize};

use crate::application::Application;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Db {
    pub entries: Vec<DbEntry>,
}

impl Db {
    const PATH: &str = "/.config/spellbook/";
    const FILENAME: &str = "spells.toml";

    pub fn load(applications: &[Application]) -> Self {
        let path = Self::get_full_path();
        let Ok(toml) = std::fs::read_to_string(&path) else {
            return Self::save_default_db(applications);
        };
        let Ok(db) = toml::from_str::<Self>(&toml) else {
            return Self::save_default_db(applications);
        };
        db
    }

    pub fn save(&self) {
        let path = Self::get_full_path();
        let toml = match toml::to_string_pretty(self) {
            Ok(toml) => toml,
            Err(err) => {
                println!("{:#?}", err);
                panic!();
            }
        };
        std::fs::create_dir_all(Self::get_path()).unwrap();
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
        let home = std::env::var("HOME").unwrap();
        format!("{}{}", home, Self::PATH)
    }

    fn get_full_path() -> String {
        format!("{}{}", Self::get_path(), Self::FILENAME)
    }

    fn save_default_db(applications: &[Application]) -> Self {
        let mut entries = Vec::new();
        applications
            .iter()
            .for_each(|a| entries.push(DbEntry::new(&a.name)));
        let db = Db { entries };
        db.save();
        db
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
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
