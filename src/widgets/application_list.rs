use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    widgets::{
        List, ListDirection, ListState, Scrollbar, ScrollbarOrientation, ScrollbarState,
        StatefulWidget,
    },
};

use crate::application::Application;

use super::counter::CounterState;

pub struct ApplicationList;

impl StatefulWidget for ApplicationList {
    type State = ApplicationListState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let [_, scrollbar_area] =
            Layout::horizontal([Constraint::Min(1), Constraint::Max(1)]).areas(area);

        let mut highlighted_and_filtered_applications = Vec::new();
        for application in &state.filtered_applications {
            let highlighted_name = application.get_highlighted_name(state.filter.as_str());
            highlighted_and_filtered_applications.push(highlighted_name);
        }

        let list = List::new(highlighted_and_filtered_applications)
            .style(Style::new().fg(Color::White))
            .highlight_style(Style::new().fg(Color::Cyan).bg(Color::Black).not_reversed())
            .direction(ListDirection::TopToBottom);

        if let None = state.list_state.selected() {
            state.list_state.select_first();
        }
        StatefulWidget::render(list, area, buf, &mut state.list_state);

        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(None)
            .end_symbol(None)
            .track_symbol(None)
            .thumb_symbol("┃")
            .style(Style::new().fg(Color::White));

        let scrollable_range =
            (state.filtered_applications.len() as i16 - area.height as i16 + 3).max(0);

        state.scrollbar_state = state
            .scrollbar_state
            .content_length(scrollable_range as usize)
            .position(state.list_state.offset());

        StatefulWidget::render(scrollbar, scrollbar_area, buf, &mut state.scrollbar_state);
    }
}

#[derive(Debug)]
pub struct ApplicationListState {
    filtered_applications: Vec<Application>,
    applications: Vec<Application>,
    filter: String,
    list_state: ListState,
    scrollbar_state: ScrollbarState,
}

impl ApplicationListState {
    pub fn default() -> Self {
        let applications = Application::find_all();
        Self {
            filtered_applications: applications.clone(),
            applications,
            filter: String::new(),
            list_state: ListState::default(),
            scrollbar_state: ScrollbarState::default(),
        }
    }

    pub fn update_filter(&mut self, filter: &str) {
        if self.filter == filter {
            return;
        }
        self.filter = filter.to_string();
        let mut filtered_applications = self
            .applications
            .clone()
            .into_iter()
            .filter(|entry| {
                entry
                    .name
                    .to_lowercase()
                    .contains(&self.filter.to_lowercase())
            })
            .collect::<Vec<Application>>();
        filtered_applications.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        self.filtered_applications = filtered_applications;
    }

    pub fn get_counter_state(&self) -> CounterState {
        CounterState::new(self.filtered_applications.len(), self.applications.len())
    }

    pub fn selected(&self) -> Option<&Application> {
        let Some(i) = self.list_state.selected() else {
            return None;
        };
        Some(&self.filtered_applications[i])
    }

    pub fn select_previous(&mut self) {
        self.list_state.select_previous();
    }

    pub fn select_next(&mut self) {
        self.list_state.select_next();
    }
}
