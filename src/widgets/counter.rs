use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::Style,
    widgets::{Paragraph, Widget},
};

use crate::config::Config;

pub struct Counter<'a> {
    config: &'a Config,
    current: usize,
    max: usize,
}

impl<'a> Counter<'a> {
    pub fn new(config: &'a Config, current: usize, max: usize) -> Self {
        Self {
            config,
            current,
            max,
        }
    }
}

impl Widget for Counter<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if !self.config.counter.enable {
            return;
        }
        let text = format!("{} / {}", self.current, self.max);
        let mut style = Style::new();
        if self.config.counter.bold {
            style = style.bold();
        }
        let paragraph = Paragraph::new(text.as_str())
            .style(style)
            .alignment(Alignment::Right);
        Widget::render(paragraph, area, buf);
    }
}
