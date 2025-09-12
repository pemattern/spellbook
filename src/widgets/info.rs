use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Paragraph, StatefulWidget, Widget},
};

use crate::config::Config;

pub struct Info<'a> {
    config: &'a Config,
}

impl<'a> Info<'a> {
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }
}

impl<'a> StatefulWidget for Info<'a> {
    type State = InfoState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let (fg_color, bg_color) = match self.config.color_mode {
            crate::config::ColorMode::Light => (Color::Black, Color::White),
            crate::config::ColorMode::Dark => (Color::White, Color::Black),
        };
        let message = state.message.clone().unwrap_or_default();
        let paragraph = Paragraph::new(message).style(Style::new().fg(fg_color).bg(bg_color));
        Widget::render(paragraph, area, buf);
    }
}

#[derive(Debug, Default)]
pub struct InfoState {
    message: Option<String>,
}

impl InfoState {
    pub fn update_message(&mut self, message: Option<String>) {
        self.message = message;
    }
}
