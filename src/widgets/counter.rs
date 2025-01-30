use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Paragraph, Widget},
};

use crate::launcher::LauncherState;

pub struct Counter {
    current: usize,
    max: usize,
    display: bool,
}

impl Counter {
    pub fn new(state: &LauncherState) -> Self {
        Self {
            current: state.application_list.filtered_applications.len(),
            max: state.application_list.applications.len(),
            display: state.config.counter.display,
        }
    }
}

impl Widget for Counter {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if !self.display {
            return;
        }
        let text = format!("{} / {}", self.current, self.max);
        let paragraph = Paragraph::new(text.as_str())
            .style(Style::new().fg(Color::White))
            .alignment(Alignment::Right);
        Widget::render(paragraph, area, buf);
    }
}
