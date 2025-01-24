use std::{
    env, fs,
    io::{self},
    os::unix::process::CommandExt,
    path::Path,
    process::Command,
};

use fork::{fork, Fork};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Margin, Position, Rect},
    style::{Color, Style, Stylize},
    widgets::{
        Block, List, ListDirection, ListState, Paragraph, Scrollbar, ScrollbarOrientation,
        ScrollbarState, StatefulWidget, Widget,
    },
    DefaultTerminal, Frame,
};

use crate::{config::Config, desktop_entry::DesktopEntry};

#[derive(Debug)]
pub struct App {
    config: Config,
    entries: Vec<DesktopEntry>,
    filter: String,
    cursor_index: usize,
    list_state: ListState,
    scrollbar_state: ScrollbarState,
    should_exit: bool,
}

impl App {
    pub fn new(config: Config) -> Self {
        let entries = Self::get_desktop_entries();
        Self {
            config,
            entries,
            filter: String::new(),
            cursor_index: 0,
            list_state: ListState::default(),
            scrollbar_state: ScrollbarState::default(),
            should_exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn select_entry(&mut self) {
        if let Some(i) = self.list_state.selected() {
            let entry = &self.get_filtered_entries()[i];
            let shell = env::var("SHELL").expect("unable to read $SHELL env");
            if entry.terminal {
                ratatui::restore();
                let _ = Command::new(&entry.exec).exec();
            } else {
                let output = Command::new(&shell)
                    .args(&[
                        "-c",
                        format!("ps -o ppid= -p {}", std::process::id()).as_str(),
                    ])
                    .output()
                    .expect("unable to get ppid");
                match fork() {
                    Ok(Fork::Child) => {
                        let ppid = String::from_utf8_lossy(&output.stdout);
                        let _ = Command::new(&shell)
                            .args(&["-c", "sleep .1"])
                            .output()
                            .expect("...");
                        ratatui::restore();
                        let _ = Command::new(&shell)
                            .args(&["-c", format!("kill -9 {}", ppid).as_str()])
                            .status()
                            .expect("unable to kill terminal process");
                    }
                    Ok(Fork::Parent(_)) => {
                        let _ = Command::new(&shell)
                            .args(&["-c", format!("{} & disown", &entry.exec).as_str()])
                            .exec();
                    }
                    Err(_) => panic!("fork failed"),
                }
            }
        }
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
                    .contains(&self.filter.to_lowercase())
            })
            .collect::<Vec<DesktopEntry>>();
        filtered_entries.sort_by(|a, b| a.name.cmp(&b.name));
        filtered_entries
    }

    fn draw(&mut self, frame: &mut Frame) {
        let index = self.cursor_index;
        frame.render_widget(self, frame.area());
        frame.set_cursor_position(Position::new(index as u16 + 2, 1));
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char(to_insert) => self.enter_char(to_insert),
                    KeyCode::Backspace => self.delete_char(),
                    KeyCode::Delete => self.right_delete_char(),
                    KeyCode::Left => self.move_cursor_left(),
                    KeyCode::Right => self.move_cursor_right(),
                    KeyCode::Up | KeyCode::BackTab => self.select_previous(),
                    KeyCode::Down | KeyCode::Tab => self.select_next(),
                    KeyCode::Enter => self.select_entry(),
                    KeyCode::Esc => self.should_exit = true,
                    _ => {}
                }
            }
        }
        Ok(())
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_right = self.cursor_index.saturating_sub(1);
        self.cursor_index = self.clamp_cursor(cursor_moved_right);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_left = self.cursor_index.saturating_add(1);
        self.cursor_index = self.clamp_cursor(cursor_moved_left);
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.filter.insert(index, new_char);
        self.move_cursor_right();
    }

    fn byte_index(&self) -> usize {
        self.filter
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.cursor_index)
            .unwrap_or(self.filter.len())
    }

    fn delete_char(&mut self) {
        let is_cursor_leftmost = self.cursor_index == 0;
        if is_cursor_leftmost {
            return;
        }
        let current_index = self.cursor_index;
        let from_left_to_current_index = current_index - 1;
        let before_char_to_delete = self.filter.chars().take(from_left_to_current_index);
        let after_char_to_delete = self.filter.chars().skip(current_index);
        self.filter = before_char_to_delete.chain(after_char_to_delete).collect();
        self.move_cursor_left();
    }

    fn right_delete_char(&mut self) {
        let is_cursor_rightmost = self.cursor_index == self.filter.len();
        if is_cursor_rightmost {
            return;
        }
        let cursor_index = self.cursor_index;
        self.filter.remove(cursor_index);
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.filter.chars().count())
    }

    fn select_previous(&mut self) {
        self.list_state.select_previous();
    }

    fn select_next(&mut self) {
        self.list_state.select_next();
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
        let main_block = Block::bordered();
        let [top_area, divider_area, list_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Min(1),
        ])
        .areas(area.inner(Margin::new(1, 1)));
        let filtered_entries = self.get_filtered_entries();
        let count_display = format!("{}/{}", filtered_entries.len(), self.entries.len());
        let [filter_area, count_area] = Layout::horizontal([
            Constraint::Min(1),
            Constraint::Length(count_display.len() as u16),
        ])
        .areas(top_area.inner(Margin::new(1, 0)));
        let [_, scrollbar_area] =
            Layout::horizontal([Constraint::Min(1), Constraint::Max(1)]).areas(list_area);
        let input = match &self.config.placeholder {
            Some(placeholder) => match &self.filter.len() {
                0 => Paragraph::new(placeholder.as_str())
                    .style(Style::new().fg(Color::DarkGray).italic()),
                _ => Paragraph::new(self.filter.as_str()),
            },
            None => Paragraph::new(self.filter.as_str()),
        };
        let count = Paragraph::new(count_display).style(Style::new().fg(Color::White));
        let divider = Paragraph::new((0..divider_area.width).map(|_| '—').collect::<String>());

        let mut highlighted_and_filtered_entries = Vec::new();
        let filtered_entries = self.get_filtered_entries();
        for entry in &filtered_entries {
            let highlighted_name = entry.get_highlighted_name(self.filter.as_str());
            highlighted_and_filtered_entries.push(highlighted_name);
        }

        let list = List::new(highlighted_and_filtered_entries)
            .style(Style::new().fg(Color::White))
            .highlight_style(Style::new().fg(Color::Cyan).bg(Color::Black).not_reversed())
            .direction(ListDirection::TopToBottom);

        if let None = self.list_state.selected() {
            self.list_state.select_first();
        }

        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(None)
            .end_symbol(None)
            .track_symbol(None)
            .thumb_symbol("┃");

        let scrollable_range = (filtered_entries.len() as i16 - area.height as i16 + 3).max(0);

        self.scrollbar_state = self
            .scrollbar_state
            .content_length(scrollable_range as usize)
            .position(self.list_state.offset());

        Widget::render(main_block, area, buf);
        Widget::render(input, filter_area, buf);
        Widget::render(count, count_area, buf);
        Widget::render(divider, divider_area, buf);
        StatefulWidget::render(list, list_area, buf, &mut self.list_state);
        StatefulWidget::render(scrollbar, scrollbar_area, buf, &mut self.scrollbar_state);
    }
}
