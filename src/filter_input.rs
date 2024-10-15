use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{Block, Paragraph, Widget},
};

#[derive(Debug)]
pub struct FilterInput {
    filter: String,
    cursor_index: usize,
    label: String,
}

impl FilterInput {
    pub fn new(label: String) -> Self {
        Self {
            filter: String::new(),
            cursor_index: 0,
            label,
        }
    }

    pub fn get_filter(&self) -> String {
        self.filter.clone()
    }

    pub fn get_cursor_index(&self) -> usize {
        self.cursor_index.clone()
    }

    pub fn move_cursor_left(&mut self) {
        let cursor_moved_right = self.cursor_index.saturating_sub(1);
        self.cursor_index = self.clamp_cursor(cursor_moved_right);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_left = self.cursor_index.saturating_add(1);
        self.cursor_index = self.clamp_cursor(cursor_moved_left);
    }

    pub fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.filter.insert(index, new_char);
        self.move_cursor_right();
    }

    pub fn byte_index(&self) -> usize {
        self.filter
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.cursor_index)
            .unwrap_or(self.filter.len())
    }

    pub fn delete_char(&mut self) {
        let is_cursor_leftmost = self.cursor_index == 0;
        if is_cursor_leftmost {
            return;
        }
        let current_index = self.cursor_index;
        let from_left_to_current_index = current_index - 1;
        let before_char_to_delete = self.filter.chars().take(from_left_to_current_index);
        let after_char_to_delete = self.filter.chars().skip(current_index);
        self.filter = before_char_to_delete.chain(after_char_to_delete).collect();
        self.move_cursor_left();
    }

    pub fn right_delete_char(&mut self) {
        let is_cursor_rightmost = self.cursor_index == self.filter.len();
        if is_cursor_rightmost {
            return;
        }
        let cursor_index = self.cursor_index;
        self.filter.remove(cursor_index);
    }

    pub fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.filter.chars().count())
    }
}

impl Widget for &mut FilterInput {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let input =
            Paragraph::new(self.filter.clone()).block(Block::bordered().title(self.label.clone()));
        Widget::render(input, area, buf);
    }
}
