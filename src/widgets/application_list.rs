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
            .filter(|entry| {
                entry.name.to_lowercase().contains(&filter.to_lowercase())
                    && !entry.db_entry.blacklisted
            })
            .collect();
    }

    pub fn selected(&self) -> Option<Application> {
        let i = self.list.selected()?;
        Some(self.filtered_applications[i].clone())
    }

    pub fn select_previous(&mut self) {
        self.list.scroll_up_by(1);
    }

    pub fn select_next(&mut self) {
        if let Some(i) = self.list.selected()
            && i < self.filtered_applications.len() - 1
        {
            self.list.scroll_down_by(1);
        }
    }

    pub fn increment_launch_count(&mut self, filtered_application: &Application) {
        self.matched_applications_mut(filtered_application)
            .for_each(|application| application.db_entry.launch_count += 1)
    }

    pub fn blacklist(&mut self, filtered_application: &Application) {
        self.matched_applications_mut(filtered_application)
            .for_each(|application| application.db_entry.blacklisted = true);
    }

    fn matched_applications_mut(
        &mut self,
        filtered_application: &Application,
    ) -> impl Iterator<Item = &mut Application> {
        self.applications
            .iter_mut()
            .filter(|application| application.name == filtered_application.name)
    }

    pub fn save_db(&self) {
        let mut entries = self
            .applications
            .iter()
            .map(|entry| entry.db_entry.clone())
            .collect::<Vec<DbEntry>>();
        entries.sort_by(|a, b| a.name.as_str().cmp(b.name.as_str()));
        Db::save_to_disk(entries);
    }
}

impl Default for ApplicationListState {
    fn default() -> Self {
        let applications = Application::find_all();
        let filtered_applications = applications
            .clone()
            .into_iter()
            .filter(|application| !application.db_entry.blacklisted)
            .collect();
        Self {
            filtered_applications,
            applications,
            list: ListState::default(),
            scrollbar: ScrollbarState::default(),
        }
    }
}
