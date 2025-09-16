use crate::{
    config::ColorMode,
    db::{Db, DbEntry},
    icon::{APPLICATION_ICON_MAP, CATEGORY_ICON_MAP, Icon},
};
use ini::Ini;
use ratatui::{
    style::{Color, Style, Stylize},
    text::Span,
};
use std::{env::home_dir, ffi::CString, fs, path::Path};

#[derive(Clone, Debug)]
pub struct Application {
    pub name: String,
    pub filename: CString,
    pub args: Vec<CString>,
    pub terminal: bool,
    pub comment: Option<String>,
    pub icon: Icon,
    pub db_entry: DbEntry,
}

impl Application {
    pub fn from_file(path: &str, db: &Db) -> Option<Self> {
        let content = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(_) => return None,
        };
        let ini = match Ini::load_from_str(&content) {
            Ok(ini) => ini,
            Err(_) => return None,
        };
        let section = ini.section(Some("Desktop Entry"));
        if let Some(section) = section {
            let name = section.get("Name")?;
            let exec = section.get("Exec")?;
            let terminal = match section.get("Terminal") {
                Some("True") | Some("true") => true,
                Some("False") | Some("false") => false,
                Some(_) => {
                    return None;
                }
                None => {
                    return None;
                }
            };
            let categories = match section.get("Categories") {
                Some(categories) => categories
                    .split(';')
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
                None => Vec::new(),
            };
            let comment = section.get("Comment").map(|comment| comment.to_string());
            let icon = Self::set_icon(name, categories).clone();
            let exec_split = exec
                .split_whitespace()
                .filter(|e| !e.starts_with('%'))
                .map(|e| e.to_string())
                .collect::<Vec<String>>();
            let filename_ref = exec_split.first()?;
            let filename = CString::new(filename_ref.to_string()).unwrap();
            let mut args = Vec::new();
            if exec_split.len() > 1 {
                args = exec_split[1..]
                    .iter()
                    .map(|a| CString::new(a.to_string()).unwrap())
                    .collect::<Vec<CString>>();
            }
            let db_entry = db
                .entries
                .iter()
                .find(|entry| entry.name == name)
                .cloned()
                .unwrap_or(DbEntry::new(name));
            let name = name.to_string();

            return Some(Self {
                name,
                filename,
                args,
                terminal,
                comment,
                icon,
                db_entry,
            });
        }
        None
    }

    pub fn find_all() -> Vec<Self> {
        let mut applications = Vec::new();
        let db = Db::load();
        let home = home_dir().unwrap();
        let dirs = vec![
            String::from("/usr/share/applications/"),
            format!("{}/.local/share/applications/", home.display()),
        ];
        for dir in dirs {
            let path = Path::new(&dir);
            if path.exists() && path.is_dir() {
                for entry in fs::read_dir(path).expect("unable to read target directory") {
                    let entry = entry.expect("unable to read entry");
                    let path = entry.path();
                    if path.is_file()
                        && path.extension().and_then(|s| s.to_str()) == Some("desktop")
                    {
                        match Application::from_file(path.to_str().unwrap(), &db) {
                            Some(app) => applications.push(app),
                            None => continue,
                        }
                    }
                }
            }
        }
        applications.sort_by(|a, b| {
            b.db_entry
                .launch_count
                .cmp(&a.db_entry.launch_count)
                .then(a.name.to_lowercase().cmp(&b.name.to_lowercase()))
        });
        applications
    }

    pub fn get_icon(&self) -> Span<'_> {
        Span::styled(
            format!("{}  ", self.icon.str),
            Style::new().fg(self.icon.color),
        )
    }

    pub fn get_highlighted_name(&self, filter: &str, color_mode: &ColorMode) -> Vec<Span<'_>> {
        let mut spans = Vec::new();
        let name = &self.name;
        let indices = name
            .to_lowercase()
            .match_indices(&filter.to_lowercase())
            .map(|(index, _)| index)
            .collect::<Vec<usize>>();
        if filter.is_empty() || indices.is_empty() {
            spans.push(Span::raw(name));
            return spans;
        }
        if indices[0] > 0 {
            spans.push(Span::raw(&name[..indices[0]]));
        }
        let mut iteration = 0;
        let bg_color = match color_mode {
            ColorMode::Light => Color::White,
            ColorMode::Dark => Color::DarkGray,
        };
        for index in indices.iter() {
            let start = *index;
            let end = start + filter.len();
            spans.push(Span::raw(&name[start..end]).style(Style::new().bold().bg(bg_color)));
            let next_index: usize = if iteration < indices.len() - 1 {
                iteration += 1;
                indices[iteration]
            } else {
                name.len()
            };
            spans.push(Span::raw(&name[end..next_index]));
        }
        spans
    }

    fn set_icon(name: &str, categories: Vec<String>) -> &Icon {
        if let Some(application_icon) = APPLICATION_ICON_MAP.get(name) {
            return application_icon;
        }
        for category in categories.iter() {
            if let Some(category_icon) = CATEGORY_ICON_MAP.get(category) {
                return category_icon;
            }
        }
        &Icon::EMPTY
    }
}
