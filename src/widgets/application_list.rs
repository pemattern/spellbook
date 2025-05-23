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

use crate::{application::Application, config::ApplicationListConfig};

use super::input::InputState;

pub struct ApplicationList<'a> {
    config: &'a ApplicationListConfig,
    filter: &'a str,
}

impl<'a> ApplicationList<'a> {
    pub fn new(config: &'a ApplicationListConfig, input: &'a InputState) -> Self {
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
            if self.config.display_icons {
                highlight_spans.push(application.get_icon());
            }
            highlight_spans.append(&mut application.get_highlighted_name(self.filter));
            highlighted_applications.push(Line::from(highlight_spans));
        }

        let list = List::new(highlighted_applications)
            .style(Style::new().fg(Color::White))
            .highlight_style(Style::new().fg(Color::Cyan).bg(Color::Black).not_reversed())
            .direction(ListDirection::TopToBottom);

        if state.list.selected().is_none() {
            *state.list.selected_mut() = Some(0);
        }

        if list.len() < area.height as usize {
            *state.list.offset_mut() = 0;
        }

        let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(None)
            .end_symbol(None)
            .track_symbol(None)
            .thumb_symbol("┃")
            .style(Style::new().fg(Color::White));
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
        let filtered_applications = self
            .applications
            .clone()
            .into_iter()
            .filter(|entry| entry.name.to_lowercase().contains(&filter.to_lowercase()))
            .collect::<Vec<Application>>();
        self.filtered_applications = filtered_applications;
    }

    pub fn selected(&self) -> Option<&Application> {
        let Some(i) = self.list.selected() else {
            return None;
        };
        Some(&self.filtered_applications[i])
    }

    pub fn select_previous(&mut self) {
        self.list.select_previous();
    }

    pub fn select_next(&mut self) {
        self.list.select_next();
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
