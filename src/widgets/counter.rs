use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style, Stylize},
    widgets::{Paragraph, Widget},
};

use crate::config::{ColorMode, Config};

use super::application_list::ApplicationListState;

pub struct Counter<'a> {
    config: &'a Config,
    current: usize,
    max: usize,
}

impl<'a> Counter<'a> {
    pub fn new(config: &'a Config, application_list_state: &ApplicationListState) -> Self {
        Self {
            config,
            current: application_list_state.filtered_applications.len(),
            max: application_list_state.applications.len(),
        }
    }
}

impl Widget for Counter<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if !self.config.counter.enable {
            return;
        }
        let (fg_color, bg_color) = match self.config.color_mode {
            ColorMode::Light => (Color::Black, Color::White),
            ColorMode::Dark => (Color::White, Color::Black),
        };
        let text = format!("{} / {}", self.current, self.max);
        let mut style = Style::new().fg(fg_color).bg(bg_color);
        if self.config.counter.bold {
            style = style.bold();
        }
        let paragraph = Paragraph::new(text.as_str())
            .style(style)
            .alignment(Alignment::Right);
        Widget::render(paragraph, area, buf);
    }
}
