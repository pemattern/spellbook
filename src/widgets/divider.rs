use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Paragraph, Widget},
};

use crate::config::BorderConfig;

pub struct Divider<'a> {
    config: &'a BorderConfig,
}

impl<'a> Divider<'a> {
    pub fn new(config: &'a BorderConfig) -> Self {
        Self { config }
    }
}

impl Widget for Divider<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let paragraph = Paragraph::new(
            (0..area.width)
                .map(|_| self.config.divider_character)
                .collect::<String>(),
        )
        .style(Style::new().fg(Color::White));
        Widget::render(paragraph, area, buf);
    }
}
