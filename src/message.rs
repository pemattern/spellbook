use crossterm::event::KeyEvent;

pub enum Message {
    Input(KeyEvent),
    Redraw,
    ReloadConfig,
}
