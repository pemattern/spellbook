use std::{
    env, fs,
    io::{self},
    path::Path,
};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Position, Rect},
    widgets::Widget,
    DefaultTerminal, Frame,
};

use crate::{
    config::Config, desktop_entry::DesktopEntry, entries_list::EntriesList,
    filter_input::FilterInput,
};

#[derive(Debug)]
pub struct App {
    config: Config,
    entries: Vec<DesktopEntry>,
    filter_input: FilterInput,
    entries_list: EntriesList,
    should_exit: bool,
}

impl App {
    pub fn new(config: Config) -> Self {
        let entries = Self::get_desktop_entries();
        let filter_label = config.filter_label.clone();
        let entries_label = config.entries_label.clone();
        Self {
            config,
            entries,
            filter_input: FilterInput::new(filter_label),
            entries_list: EntriesList::new(entries_label),
            should_exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.should_exit {
            self.entries_list.set_entries(self.get_filtered_entries());
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn get_filtered_entries(&self) -> Vec<DesktopEntry> {
        let mut filtered_entries = self
            .entries
            .clone()
            .into_iter()
            .filter(|entry| {
                entry
                    .name
                    .to_lowercase()
                    .contains(&self.filter_input.get_filter().to_lowercase())
            })
            .collect::<Vec<DesktopEntry>>();
        filtered_entries.sort_by_key(|entry| entry.name.clone());
        filtered_entries
    }

    fn draw(&mut self, frame: &mut Frame) {
        let index = self.filter_input.get_cursor_index();
        frame.render_widget(self, frame.area());
        frame.set_cursor_position(Position::new(index as u16 + 1, 1));
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.modifiers == KeyModifiers::CONTROL {
                match key {
                    KeyEvent {
                        code: KeyCode::Char('k'),
                        ..
                    } => self.entries_list.select_previous(),
                    KeyEvent {
                        code: KeyCode::Char('j'),
                        ..
                    } => self.entries_list.select_next(),
                    _ => {}
                }
            } else if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char(to_insert) => self.filter_input.enter_char(to_insert),
                    KeyCode::Backspace => self.filter_input.delete_char(),
                    KeyCode::Delete => self.filter_input.right_delete_char(),
                    KeyCode::Left => self.filter_input.move_cursor_left(),
                    KeyCode::Right => self.filter_input.move_cursor_right(),
                    KeyCode::Up => self.entries_list.select_previous(),
                    KeyCode::Down => self.entries_list.select_next(),
                    KeyCode::Enter => self.entries_list.select_app(),
                    KeyCode::Esc => self.should_exit = true,
                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn get_desktop_entries() -> Vec<DesktopEntry> {
        let mut apps = Vec::new();
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
                        match DesktopEntry::from_file(path.to_str().unwrap()) {
                            Some(app) => apps.push(app),
                            None => continue,
                        }
                    }
                }
            }
        }
        apps
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [input_area, list_area] =
            Layout::vertical([Constraint::Length(3), Constraint::Min(1)]).areas(area);

        Widget::render(&mut self.filter_input, input_area, buf);
        Widget::render(&mut self.entries_list, list_area, buf);
    }
}
