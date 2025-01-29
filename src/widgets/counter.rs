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
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        if !state.config.counter.display {
            return;
        }
        let text = format!(
            "{} / {}",
            state.application_list.filtered_applications.len(),
            state.application_list.applications.len()
        );
        let paragraph = Paragraph::new(text.as_str())
            .style(Style::new().fg(Color::White))
            .alignment(Alignment::Right);
        Widget::render(paragraph, area, buf);
    }
}
