use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Paragraph, Widget},
};

use crate::launcher::LauncherState;

pub struct Divider {
    character: char,
}

impl Divider {
    pub fn new(state: &LauncherState) -> Self {
        Self {
            character: state.config.divider.character,
        }
    }
}

impl Widget for Divider {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let paragraph = Paragraph::new((0..area.width).map(|_| self.character).collect::<String>())
            .style(Style::new().fg(Color::White));
        Widget::render(paragraph, area, buf);
    }
}
