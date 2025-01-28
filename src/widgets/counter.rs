use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Paragraph, StatefulWidget, Widget},
};

use crate::config::{Config, CounterConfig};

pub struct Counter;

impl StatefulWidget for Counter {
    type State = CounterState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut CounterState) {
        if !state.config.display {
            return;
        }
        let paragraph = Paragraph::new(state.text())
            .style(Style::new().fg(Color::White))
            .alignment(Alignment::Right);
        Widget::render(paragraph, area, buf);
    }
}

#[derive(Debug)]
pub struct CounterState {
    current: usize,
    max: usize,
    pub config: CounterConfig,
}

impl CounterState {
    pub fn from_config(config: &Config) -> Self {
        Self {
            current: 0,
            max: 0,
            config: config.counter.clone(),
        }
    }

    pub fn update_counts(&mut self, counts: (usize, usize)) {
        self.current = counts.0;
        self.max = counts.1;
    }

    pub fn width(&self) -> u16 {
        self.text().len() as u16
    }

    fn text(&self) -> String {
        format!("{}/{}", self.current, self.max)
    }
}
