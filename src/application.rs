use crate::icon::{Icon, APPLICATION_ICON_MAP, CATEGORY_ICON_MAP};
use ini::Ini;
use ratatui::{
    style::{Color, Style, Stylize},
    text::{Line, Span},
};
use std::{env, fs, path::Path};

#[derive(Clone, Debug)]
pub struct Application {
    pub name: String,
    pub exec: String,
    pub terminal: bool,
    pub icon: Icon,
}

impl Application {
    pub fn from_file(path: &str) -> Option<Self> {
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
            let Some(name) = section.get("Name") else {
                return None;
            };
            let Some(exec) = section.get("Exec") else {
                return None;
            };
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
            let icon = Self::get_icon(name, categories).clone();
            return Some(Self {
                name: name.to_string(),
                exec: exec
                    .split_whitespace()
                    .filter(|s| !s.starts_with('%'))
                    .collect::<Vec<&str>>()
                    .join(" "),
                terminal,
                icon,
            });
        }
        None
    }

    pub fn find_all() -> Vec<Self> {
        let mut applications = Vec::new();
        let home = env::var("HOME").expect("unable to read $HOME env");
        let dirs = vec![
            "/usr/share/applications/".to_string(),
            format!("{}/.local/share/applications/", home),
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
                        match Application::from_file(path.to_str().unwrap()) {
                            Some(app) => applications.push(app),
                            None => continue,
                        }
                    }
                }
            }
        }
        applications.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        applications
    }

    pub fn get_highlighted_name(&self, filter: &str) -> Line {
        let mut spans = Vec::new();
        let name = &self.name;
        let indices = name
            .to_lowercase()
            .match_indices(&filter.to_lowercase())
            .map(|(index, _)| index)
            .collect::<Vec<usize>>();
        spans.push(Span::styled(
            format!(" {} ", self.icon.str),
            Style::new().fg(self.icon.color),
        ));
        if filter.len() == 0 || indices.len() == 0 {
            spans.push(Span::raw(name));
            return Line::from(spans);
        }
        if indices[0] > 0 {
            spans.push(Span::raw(&name[..indices[0]]));
        }
        let mut iteration = 0;
        for index in indices.iter() {
            let start = *index;
            let end = start + filter.len();
            spans.push(Span::raw(&name[start..end]).style(Style::new().bold().bg(Color::DarkGray)));
            let next_index: usize;
            if iteration < indices.len() - 1 {
                iteration += 1;
                next_index = indices[iteration];
            } else {
                next_index = name.len();
            }
            spans.push(Span::raw(&name[end..next_index]));
        }
        Line::from(spans)
    }

    fn get_icon(name: &str, categories: Vec<String>) -> &Icon {
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
