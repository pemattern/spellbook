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
        Block, List, ListDirection, ListState, Scrollbar, ScrollbarOrientation, ScrollbarState,
        StatefulWidget, Widget,
    },
    DefaultTerminal, Frame,
};

use crate::{
    config::Config,
    desktop_entry::DesktopEntry,
    widgets::{
        counter::Counter,
        divider::Divider,
        input::{Input, InputState},
    },
};

#[derive(Debug)]
pub struct App {
    entries: Vec<DesktopEntry>,
    input_state: InputState,
    list_state: ListState,
    scrollbar_state: ScrollbarState,
    should_exit: bool,
}

impl App {
    pub fn new(config: Config) -> Self {
        let entries = Self::get_desktop_entries();
        Self {
            entries,
            input_state: InputState::from_config(&config),
            list_state: ListState::default(),
            scrollbar_state: ScrollbarState::default(),
            should_exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.should_exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_input()?;
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let index = self.input_state.cursor_index as u16;
        frame.render_widget(self, frame.area());
        frame.set_cursor_position(Position::new(index + 2, 1));
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
                    .contains(&self.input_state.filter.to_lowercase())
            })
            .collect::<Vec<DesktopEntry>>();
        filtered_entries.sort_by(|a, b| a.name.cmp(&b.name));
        filtered_entries
    }

    fn handle_input(&mut self) -> io::Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char(to_insert) => self.input_state.enter_char(to_insert),
                    KeyCode::Backspace => self.input_state.delete_char(),
                    KeyCode::Delete => self.input_state.right_delete_char(),
                    KeyCode::Left => self.input_state.move_cursor_left(),
                    KeyCode::Right => self.input_state.move_cursor_right(),
                    KeyCode::Enter => self.select_entry(),
                    KeyCode::Down | KeyCode::Tab => self.select_next(),
                    KeyCode::Up | KeyCode::BackTab => self.select_previous(),
                    KeyCode::Esc => self.should_exit = true,
                    _ => {}
                }
            }
        }
        Ok(())
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
        let counter = Counter::new(filtered_entries.len(), self.entries.len());
        let [filter_area, _, counter_area] = Layout::horizontal([
            Constraint::Min(1),
            Constraint::Length(1),
            Constraint::Length(counter.width()),
        ])
        .areas(top_area.inner(Margin::new(1, 0)));
        let [_, scrollbar_area] =
            Layout::horizontal([Constraint::Min(1), Constraint::Max(1)]).areas(list_area);

        let divider = Divider::new('─');

        let mut highlighted_and_filtered_entries = Vec::new();
        for entry in &filtered_entries {
            let highlighted_name = entry.get_highlighted_name(self.input_state.filter.as_str());
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
            .thumb_symbol("┃")
            .style(Style::new().fg(Color::White));

        let scrollable_range = (filtered_entries.len() as i16 - area.height as i16 + 3).max(0);

        self.scrollbar_state = self
            .scrollbar_state
            .content_length(scrollable_range as usize)
            .position(self.list_state.offset());

        Widget::render(main_block, area, buf);
        StatefulWidget::render(Input, filter_area, buf, &mut self.input_state);
        Widget::render(counter, counter_area, buf);
        Widget::render(divider, divider_area, buf);
        StatefulWidget::render(list, list_area, buf, &mut self.list_state);
        StatefulWidget::render(scrollbar, scrollbar_area, buf, &mut self.scrollbar_state);
    }
}
