use serde::{Deserialize, Serialize};

use crate::application::{self, Application};

pub struct Db;

impl Db {
    const PATH: &str = "/Dev/spellbook/spells.toml";

    pub fn load(applications: &Vec<Application>) -> Vec<DbEntry> {
        let path = Self::get_path();
        match std::fs::read_to_string(&path) {
            Ok(toml) => toml::from_str::<Vec<DbEntry>>(&toml).unwrap(),
            Err(_) => Self::default_db(applications),
        }
    }

    fn get_path() -> String {
        let home = std::env::var("HOME").unwrap();
        format!("{}{}", home, Self::PATH)
    }

    fn default_db(applications: &Vec<Application>) -> Vec<DbEntry> {
        let mut entries = Vec::new();
        applications
            .iter()
            .for_each(|a| entries.push(DbEntry::new(&a.name)));
        entries
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields, rename = "entry")]
pub struct DbEntry {
    name: String,
    launch_count: usize,
    blacklisted: bool,
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
