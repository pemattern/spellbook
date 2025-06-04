use std::io::Write;

use serde::{Deserialize, Serialize};

use crate::application::Application;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct Db {
    pub entries: Vec<DbEntry>,
}

impl Db {
    const PATH: &str = "/Dev/spellbook/spells.toml";

    pub fn load(applications: &Vec<Application>) -> Self {
        let path = Self::get_path();
        match std::fs::read_to_string(&path) {
            Ok(toml) => toml::from_str::<Self>(&toml).unwrap(),
            Err(_) => {
                let db = Self::default_db(applications);
                db.save();
                db
            }
        }
    }

    pub fn save(&self) {
        let path = Self::get_path();
        let toml = match toml::to_string_pretty(self) {
            Ok(toml) => toml,
            Err(err) => {
                println!("{:#?}", err);
                panic!();
            }
        };
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

    fn default_db(applications: &Vec<Application>) -> Self {
        let mut entries = Vec::new();
        applications
            .iter()
            .for_each(|a| entries.push(DbEntry::new(&a.name)));
        Db { entries }
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
