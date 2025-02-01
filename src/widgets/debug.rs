use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Style, Stylize},
    widgets::{Paragraph, Widget},
};

use crate::launcher::LauncherState;

pub struct Debug<'a> {
    state: &'a LauncherState,
}

impl<'a> Debug<'a> {
    pub fn new(state: &'a LauncherState) -> Self {
        Self { state }
    }
}

impl<'a> Widget for Debug<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let paragraph =
            Paragraph::new(self.state.debug.message.as_str()).style(Style::new().bold());
        Widget::render(paragraph, area, buf);
    }
}

#[derive(Debug, Default)]
pub struct DebugState {
    message: String,
}

impl DebugState {
    pub fn log(&mut self, message: String) {
        self.message = message;
    }
}
