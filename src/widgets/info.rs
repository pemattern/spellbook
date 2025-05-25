use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    widgets::{Paragraph, Widget},
};

use crate::{application::Application, config::Config};

pub struct Info<'a> {
    config: &'a Config,
    application: Option<&'a Application>,
}

impl<'a> Info<'a> {
    pub fn new(config: &'a Config, application: Option<&'a Application>) -> Self {
        Self {
            config,
            application,
        }
    }
}

impl<'a> Widget for Info<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (fg_color, bg_color) = match self.config.color_mode {
            crate::config::ColorMode::Light => (Color::Black, Color::White),
            crate::config::ColorMode::Dark => (Color::White, Color::Black),
        };
        let message = if self.application.is_some() {
            self.application.unwrap().name.as_str()
        } else {
            ""
        };
        let paragraph = Paragraph::new(message).style(Style::new().fg(fg_color).bg(bg_color));
        Widget::render(paragraph, area, buf);
    }
}
