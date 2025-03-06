use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KeyEvent {
    pub key_name: KeyName,
    pub key_type: KeyType,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeyType {
    Down,
    Up,
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeyName {
    Normal(String),
    Ctrl,
    Alt,
    Shift,
    Tab,
    Enter,
    Delete,
    Esc,
}

impl KeyEvent {
    pub fn from_str(str_from_keyboard: &str) -> Result<Self> {
        let key_event_temp: serde_json::Result<KeyEventTemp> =
            serde_json::from_str(str_from_keyboard);
        match key_event_temp {
            Ok(o) => Self::from_key_event_temp(o),
            Err(e) => Err(e.into()),
        }
    }
    fn from_key_event_temp(key_event_temp: KeyEventTemp) -> Result<Self> {
        let key_name = match key_event_temp.key_name.as_str() {
            "ctrl" => KeyName::Ctrl,
            "alt" => KeyName::Alt,
            "shift" => KeyName::Shift,
            "tab" => KeyName::Tab,
            "enter" => KeyName::Enter,
            "delete" => KeyName::Delete,
            "esc" => KeyName::Esc,
            c => KeyName::Normal(c.to_string()),
        };
        let key_type = match key_event_temp.key_type.as_str() {
            "down" => KeyType::Down,
            "up" => KeyType::Up,
            _ => {
                return Err(Error::msg(
                    "error in key_event_temp.key_type,it can only to be down or up",
                ));
            }
        };
        Ok(Self { key_name, key_type })
    }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KeyEventTemp {
    #[serde(rename = "name")]
    key_name: String,
    key_type: String,
}
