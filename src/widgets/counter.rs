use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Paragraph, StatefulWidget, Widget},
};

use crate::launcher::LauncherState;

pub struct Counter;

impl StatefulWidget for Counter {
    type State = LauncherState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut LauncherState) {
        if !state.config.counter.display {
            return;
        }
        let paragraph = Paragraph::new(state.counter.text())
            .style(Style::new().fg(Color::White))
            .alignment(Alignment::Right);
        Widget::render(paragraph, area, buf);
    }
}

#[derive(Debug, Default)]
pub struct CounterState {
    text: String,
}
