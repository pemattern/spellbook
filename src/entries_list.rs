use std::{env, os::unix::process::CommandExt, process::Command};

use fork::{fork, Fork};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style},
    widgets::{
        Block, List, ListDirection, ListState, Scrollbar, ScrollbarOrientation, ScrollbarState,
        StatefulWidget, Widget,
    },
};

use crate::desktop_entry::DesktopEntry;

#[derive(Debug)]
pub struct EntriesList {
    entries: Vec<DesktopEntry>,
    list_state: ListState,
    scrollbar_state: ScrollbarState,
    label: String,
}

impl EntriesList {
    pub fn new(label: String) -> Self {
        Self {
            entries: Vec::new(),
            list_state: ListState::default(),
            scrollbar_state: ScrollbarState::default(),
            label,
        }
    }

    pub fn set_entries(&mut self, entries: Vec<DesktopEntry>) {
        self.entries = entries;
    }

    pub fn select_previous(&mut self) {
        self.list_state.select_previous();
    }

    pub fn select_next(&mut self) {
        self.list_state.select_next();
    }

    pub fn select_app(&mut self) {
        if let Some(i) = self.list_state.selected() {
            let app = &self.entries[i];
            let shell = env::var("SHELL").expect("unable to read $SHELL env");
            if app.terminal {
                ratatui::restore();
                let _ = Command::new(&app.exec).exec();
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
                            .args(&["-c", format!("{} & disown", &app.exec).as_str()])
                            .exec();
                    }
                    Err(_) => panic!("fork failed"),
                }
            }
        }
    }
}

impl Widget for &mut EntriesList {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [_, scrollbar_area] = Layout::horizontal([Constraint::Min(1), Constraint::Max(1)])
            .margin(1)
            .areas(area);

        let list = List::new(
            self.entries
                .iter()
                .map(|app| format!(" {} {}", &app.icon, &app.name))
                .collect::<Vec<String>>(),
        )
        .block(Block::bordered().title(self.label.clone()))
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default().fg(Color::Black).bg(Color::White))
        .direction(ListDirection::TopToBottom);

        if let None = self.list_state.selected() {
            self.list_state.select_first();
        }

        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(None)
            .end_symbol(None)
            .track_symbol(None)
            .thumb_symbol("┃");

        let scrollable_range = (self.entries.len() as i16 - area.height as i16 + 3).max(0);

        self.scrollbar_state = self
            .scrollbar_state
            .content_length(scrollable_range as usize)
            .position(self.list_state.offset());

        StatefulWidget::render(list, area, buf, &mut self.list_state);
        StatefulWidget::render(scrollbar, scrollbar_area, buf, &mut self.scrollbar_state);
    }
}
