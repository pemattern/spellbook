use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{
        List, ListDirection, ListState, Scrollbar, ScrollbarOrientation, ScrollbarState,
        StatefulWidget,
    },
};

use crate::{
    application::Application,
    config::Config,
    db::{Db, DbEntry},
};

use super::input::InputState;

pub struct ApplicationList<'a> {
    config: &'a Config,
    filter: &'a str,
}

impl<'a> ApplicationList<'a> {
    pub fn new(config: &'a Config, input: &'a InputState) -> Self {
        Self {
            config,
            filter: &input.filter,
        }
    }
}

impl StatefulWidget for ApplicationList<'_> {
    type State = ApplicationListState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let [_, scrollbar_area] =
            Layout::horizontal([Constraint::Min(1), Constraint::Max(1)]).areas(area);

        let mut highlighted_applications = Vec::new();
        for application in &state.filtered_applications {
            let mut highlight_spans = vec![Span::from(" ")];
            if self.config.application_list.display_icons {
                highlight_spans.push(application.get_icon());
            }
            highlight_spans.append(
                &mut application.get_highlighted_name(self.filter, &self.config.color_mode),
            );
            highlighted_applications.push(Line::from(highlight_spans));
        }

        let (fg_color, bg_color, highlight_color) = match self.config.color_mode {
            crate::config::ColorMode::Light => (Color::Black, Color::Gray, Color::White),
            crate::config::ColorMode::Dark => (Color::White, Color::Reset, Color::Black),
        };
        let list = List::new(highlighted_applications)
            .style(Style::new().fg(fg_color).bg(bg_color))
            .highlight_style(
                Style::new()
                    .fg(Color::Cyan)
                    .bg(highlight_color)
                    .not_reversed(),
            )
            .direction(ListDirection::TopToBottom);

        if state.list.selected().is_none() {
            *state.list.selected_mut() = Some(0);
        }

        if list.len() <= area.height.into() {
            *state.list.offset_mut() = 0;
        }

        while list.len() > area.height.into()
            && list.len() - state.list.offset() < area.height.into()
        {
            *state.list.offset_mut() -= 1;
        }
        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(None)
            .end_symbol(None)
            .track_symbol(None)
            .thumb_symbol("┃")
            .style(Style::new().fg(fg_color));
        let scrollable_range =
            (state.filtered_applications.len() as i16 - area.height as i16).max(0);
        let mut scrollbar_state = state
            .scrollbar
            .content_length(scrollable_range as usize)
            .position(state.list.offset());

        StatefulWidget::render(list, area, buf, &mut state.list);
        StatefulWidget::render(scrollbar, scrollbar_area, buf, &mut scrollbar_state);
    }
}

#[derive(Debug)]
pub struct ApplicationListState {
    pub filtered_applications: Vec<Application>,
    pub applications: Vec<Application>,
    list: ListState,
    scrollbar: ScrollbarState,
}

impl ApplicationListState {
    pub fn update(&mut self, filter: &str) {
        self.filtered_applications = self
            .applications
            .clone()
            .into_iter()
            .filter(|entry| entry.name.to_lowercase().contains(&filter.to_lowercase()))
            .collect::<Vec<Application>>();
    }

    pub fn selected(&self) -> Option<Application> {
        let i = self.list.selected()?;
        Some(self.filtered_applications[i].clone())
    }

    pub fn select_previous(&mut self) {
        self.list.select_previous();
    }

    pub fn select_next(&mut self) {
        self.list.select_next();
    }

    pub fn increment_launch_count(&mut self, filtered_application: &Application) {
        self.applications
            .iter_mut()
            .filter(|application| application.name == filtered_application.name)
            .for_each(|application| application.db_entry.launch_count += 1)
    }

    pub fn save_db(&self) {
        let entries = self
            .applications
            .iter()
            .map(|entry| entry.db_entry.clone())
            .collect::<Vec<DbEntry>>();
        let db = Db { entries };
        db.save_to_disk();
    }
}

impl Default for ApplicationListState {
    fn default() -> Self {
        let applications = Application::find_all();
        Self {
            filtered_applications: applications.clone(),
            applications,
            list: ListState::default(),
            scrollbar: ScrollbarState::default(),
        }
    }
}
