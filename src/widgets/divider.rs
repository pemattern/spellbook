use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Paragraph, Widget},
};

use crate::config::DividerConfig;

pub struct Divider<'a> {
    config: &'a DividerConfig,
}

impl<'a> Divider<'a> {
    pub fn new(config: &'a DividerConfig) -> Self {
        Self { config }
    }
}

impl<'a> Widget for Divider<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let paragraph = Paragraph::new(
            (0..area.width)
                .map(|_| self.config.character)
                .collect::<String>(),
        )
        .style(Style::new().fg(Color::White));
        Widget::render(paragraph, area, buf);
    }
}
