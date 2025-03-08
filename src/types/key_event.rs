use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};

/// 按键事件结构体，包含按键名称和按键类型
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KeyEvent {
    /// 按键的名称，具体类型由 `KeyName` 枚举定义
    pub key_name: KeyName,
    /// 按键的类型，如按下或释放，具体类型由 `KeyType` 枚举定义
    pub key_type: KeyType,
}
/// 定义按键的类型，目前支持按下和释放两种状态
///
/// 该枚举类型用于表示按键的状态，包括按下（`Down`）和释放（`Up`）。
/// 它支持克隆、复制、比较、哈希和序列化等操作。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeyType {
    /// 表示按键被按下的状态
    Down,
    /// 表示按键被释放的状态
    Up,
}

/// 定义按键名称的枚举类型  
///
/// 该枚举类型用于表示不同类型的按键，包括普通按键和一些特殊功能按键。  
/// 支持克隆、复制、比较、哈希和序列化等操作。
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KeyName {
    /// 表示普通按键，其值为按键名称的字符串
    Normal(String),
    /// 表示 Ctrl 键
    Ctrl,
    /// 表示 Alt 键
    Alt,
    /// 表示 Shift 键
    Shift,
    /// 表示 Tab 键
    Tab,
    /// 表示 Enter 键
    Enter,
    /// 表示 Delete 键
    Delete,
    /// 表示 Esc 键
    Esc,
}

impl KeyEvent {
    /// 从字符串解析出 `KeyEvent` 实例
    ///
    /// 此函数接收一个表示按键事件的 JSON 字符串，尝试将其解析为 `KeyEventTemp` 结构体，
    /// 然后调用 `from_key_event_temp` 方法将 `KeyEventTemp` 转换为 `KeyEvent` 实例。
    ///
    /// # 参数
    /// - `str_from_keyboard`: 包含按键事件信息的 JSON 字符串。
    ///
    /// # 返回值
    /// - 如果解析成功，返回一个包含 `KeyEvent` 实例的 `Result`。
    /// - 如果解析失败，返回一个包含错误信息的 `Result`。
    pub fn from_str(str_from_keyboard: &str) -> Result<Self> {
        // 尝试将传入的 JSON 字符串解析为 KeyEventTemp 结构体
        let key_event_temp: serde_json::Result<KeyEventTemp> =
            serde_json::from_str(str_from_keyboard);
        // 根据解析结果进行处理
        match key_event_temp {
            // 解析成功，调用 from_key_event_temp 方法将 KeyEventTemp 转换为 KeyEvent 实例
            Ok(o) => Self::from_key_event_temp(o),
            // 解析失败，将错误信息包装成 Result 类型返回
            Err(e) => Err(e.into()),
        }
    }
    /// 从 `KeyEventTemp` 结构体创建 `KeyEvent` 实例
    ///
    /// 此函数接收一个 `KeyEventTemp` 结构体，将其内部的字符串类型的按键名称和按键类型
    /// 转换为对应的 `KeyName` 枚举和 `KeyType` 枚举值，从而创建一个 `KeyEvent` 实例。
    ///
    /// # 参数
    /// - `key_event_temp`: 包含按键事件信息的 `KeyEventTemp` 结构体。
    ///
    /// # 返回值
    /// - 如果转换成功，返回一个包含 `KeyEvent` 实例的 `Result`。
    /// - 如果 `key_event_temp.key_type` 不是 "down" 或 "up"，返回一个包含错误信息的 `Result`。
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
/// 临时按键事件结构体，用于从 JSON 字符串解析按键事件
///
/// 该结构体包含按键名称和按键类型，使用 `serde` 进行序列化和反序列化。
/// 其中 `key_name` 字段在序列化和反序列化时使用 `name` 作为 JSON 字段名。
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KeyEventTemp {
    /// 按键名称，在 JSON 中对应的字段名为 `name`
    #[serde(rename = "name")]
    key_name: String,
    /// 按键类型，以字符串形式表示
    key_type: String,
}
