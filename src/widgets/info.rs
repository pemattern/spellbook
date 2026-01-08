use ratatui::{
    buffer::Buffer,
    layout::Rect,
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
        if !self.config.info.enable {
            return;
        }
        let message = state.message.clone().unwrap_or_default();
        let paragraph = Paragraph::new(message);
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
