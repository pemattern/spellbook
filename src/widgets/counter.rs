use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Paragraph, Widget},
};

pub struct Counter {
    text: String,
}

impl Counter {
    pub fn new(current: usize, max: usize) -> Self {
        Self {
            text: format!("{}/{}", current, max),
        }
    }

    pub fn width(&self) -> u16 {
        self.text.len() as u16
    }
}

impl Widget for Counter {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let paragraph = Paragraph::new(self.text)
            .style(Style::new().fg(Color::White))
            .alignment(Alignment::Right);
        Widget::render(paragraph, area, buf);
    }
}
