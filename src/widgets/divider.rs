use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Paragraph, StatefulWidget, Widget},
};

use crate::launcher::LauncherState;

pub struct Divider;

impl StatefulWidget for Divider {
    type State = LauncherState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let paragraph = Paragraph::new(
            (0..area.width)
                .map(|_| state.divider.character)
                .collect::<String>(),
        )
        .style(Style::new().fg(Color::White));
        Widget::render(paragraph, area, buf);
    }
}

#[derive(Debug)]
pub struct DividerState {
    character: char,
}

impl Default for DividerState {
    fn default() -> Self {
        Self { character: '─' }
    }
}
