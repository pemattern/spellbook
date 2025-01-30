use crossterm::event::KeyCode;

pub enum Message {
    Input(KeyCode),
    Redraw,
    ReloadConfig,
}
