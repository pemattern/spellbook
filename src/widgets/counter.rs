use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Paragraph, StatefulWidget, Widget},
};

pub struct Counter;

impl StatefulWidget for Counter {
    type State = CounterState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut CounterState) {
        let paragraph = Paragraph::new(state.text())
            .style(Style::new().fg(Color::White))
            .alignment(Alignment::Right);
        Widget::render(paragraph, area, buf);
    }
}

#[derive(Debug, Default)]
pub struct CounterState {
    current: usize,
    max: usize,
}

impl CounterState {
    pub fn new(current: usize, max: usize) -> Self {
        Self { current, max }
    }

    pub fn width(&self) -> u16 {
        self.text().len() as u16
    }

    fn text(&self) -> String {
        format!("{}/{}", self.current, self.max)
    }
}
