use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use serde::{Deserialize, Serialize};

// TODO: implement custom serializer/deserializer
// [keybind]
// exit = "esc"
// launch = "enter"
// launch_keep_alive = "alt+enter"
#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Keybind {
    pub modifier: Option<KeybindModifier>,
    pub key: KeybindTrigger,
}

impl Keybind {
    pub fn new(key: KeybindTrigger) -> Self {
        Self {
            modifier: None,
            key,
        }
    }

    pub fn new_with_mod(r#mod: KeybindModifier, trigger: KeybindTrigger) -> Self {
        Self {
            modifier: Some(r#mod),
            key: trigger,
        }
    }
}

impl TryFrom<KeyEvent> for Keybind {
    type Error = ();
    fn try_from(value: KeyEvent) -> Result<Self, Self::Error> {
        let modifier = match value.modifiers {
            KeyModifiers::SHIFT => Some(KeybindModifier::Shift),
            KeyModifiers::ALT => Some(KeybindModifier::Alt),
            KeyModifiers::CONTROL => Some(KeybindModifier::Ctrl),
            KeyModifiers::SUPER => Some(KeybindModifier::Super),
            _ => None,
        };
        let key = match value.code {
            KeyCode::Esc => KeybindTrigger::Esc,
            KeyCode::Tab => KeybindTrigger::Tab,
            KeyCode::Enter => KeybindTrigger::Enter,
            KeyCode::Delete => KeybindTrigger::Delete,
            _ => return Err(()),
        };
        Ok(Self { modifier, key })
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum KeybindModifier {
    Shift,
    Alt,
    Ctrl,
    Super,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum KeybindTrigger {
    Esc,
    Tab,
    Enter,
    Delete,
}
