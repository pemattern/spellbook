use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style, Stylize},
    widgets::{Paragraph, Widget},
};

use crate::config::CounterConfig;

use super::application_list::ApplicationListState;

pub struct Counter<'a> {
    config: &'a CounterConfig,
    current: usize,
    max: usize,
}

impl<'a> Counter<'a> {
    pub fn new(config: &'a CounterConfig, application_list_state: &ApplicationListState) -> Self {
        Self {
            config,
            current: application_list_state.filtered_applications.len(),
            max: application_list_state.applications.len(),
        }
    }
}

impl Widget for Counter<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if !self.config.enable {
            return;
        }
        let text = format!("{} / {}", self.current, self.max);
        let mut style = Style::new().fg(Color::White).bg(Color::Black);
        if self.config.bold {
            style = style.bold();
        }
        let paragraph = Paragraph::new(text.as_str())
            .style(style)
            .alignment(Alignment::Right);
        Widget::render(paragraph, area, buf);
    }
}
