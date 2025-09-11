use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::{config::KeybindConfig, keybind::Keybind};

pub enum Action {
    None,
    Exit,
    Launch,
    LaunchKeepAlive,
    EnterChar(char),
    RemovePreviousChar,
    RemoveNextChar,
    MoveCursorLeft,
    MoveCursorRight,
    SelectNextApplication,
    SelectPreviousApplication,
    Blacklist,
}

impl Action {
    pub fn from_key_event(key_event: KeyEvent, config: &KeybindConfig) -> Self {
        match (key_event.modifiers, key_event.code) {
            (_, KeyCode::Char(to_insert)) => Action::EnterChar(to_insert),
            (KeyModifiers::NONE, KeyCode::Backspace) => Action::RemovePreviousChar,
            (KeyModifiers::NONE, KeyCode::Delete) => Action::RemoveNextChar,
            (KeyModifiers::NONE, KeyCode::Left) => Action::MoveCursorLeft,
            (KeyModifiers::NONE, KeyCode::Right) => Action::MoveCursorRight,
            (KeyModifiers::NONE, KeyCode::Down) | (KeyModifiers::NONE, KeyCode::Tab) => {
                Action::SelectNextApplication
            }
            (KeyModifiers::NONE, KeyCode::Up) | (KeyModifiers::SHIFT, KeyCode::BackTab) => {
                Action::SelectPreviousApplication
            }
            _ => {
                let Ok(keybind) = Keybind::try_from(key_event) else {
                    return Action::None;
                };
                if keybind == config.exit {
                    return Action::Exit;
                } else if keybind == config.launch {
                    return Action::Launch;
                } else if keybind == config.launch_keep_alive {
                    return Action::LaunchKeepAlive;
                } else if keybind == config.blacklist {
                    return Action::Blacklist;
                }
                Action::None
            }
        }
    }
}
