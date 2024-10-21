use crate::icons::{APPLICATION_ICON_MAP, CATEGORY_ICON_MAP};
use ini::Ini;
use ratatui::{
    style::{Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::ListItem,
};
use std::fs;

#[derive(Clone, Debug)]
pub struct DesktopEntry {
    pub name: String,
    pub exec: String,
    pub terminal: bool,
    pub icon: String,
}

impl DesktopEntry {
    pub fn from_file(path: &str) -> Option<DesktopEntry> {
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
            let icon = Self::get_icon(name, categories);
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

    pub fn get_highlighted_name(&self, filter: String) -> Line {
        let mut spans = Vec::new();
        let name = &self.name;
        let indices = name
            .to_lowercase()
            .match_indices(&filter.to_lowercase())
            .map(|(index, _)| index)
            .collect::<Vec<usize>>();
        spans.push(Span::from(format!(" {} ", self.icon)));
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
            spans.push(Span::raw(&name[start..end]).bold().reversed());
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

    fn get_icon(name: &str, categories: Vec<String>) -> String {
        let mut i = 0;
        while i < APPLICATION_ICON_MAP.len() {
            if APPLICATION_ICON_MAP[i].0.to_lowercase() == name.to_lowercase() {
                return APPLICATION_ICON_MAP[i].1.to_string();
            }
            i += 1;
        }
        i = 0;
        while i < CATEGORY_ICON_MAP.len() {
            for category in categories.iter() {
                if CATEGORY_ICON_MAP[i].0.to_lowercase() == category.to_lowercase() {
                    return CATEGORY_ICON_MAP[i].1.to_string();
                }
            }
            i += 1;
        }
        " ".to_string()
        // "󰍹".to_string()
    }
}
