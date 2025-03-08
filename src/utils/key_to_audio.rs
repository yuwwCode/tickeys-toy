use std::collections::HashMap;

use kira::sound::static_sound::StaticSoundData;

use crate::types::key_event::{KeyEvent, KeyName};
use rand::{self, seq::IndexedRandom};
/// 将输入键盘事件转换为音频数据
///
/// 该函数根据传入的键盘事件和对应的音频数据映射，随机选择一个合适的音频数据。
/// 如果指定按键的音频数据不存在，则尝试使用通用的 `normal` 音频数据。
/// 如果仍然没有可用的音频数据，则会触发 `panic`。
///
/// # 参数
/// - `key`: 表示键盘事件的结构体，包含按键类型和按键名称等信息。
/// - `audios_down`: 按下按键时对应的音频数据映射，键为按键名称，值为音频数据列表。
/// - `audios_up`: 释放按键时对应的音频数据映射，键为按键名称，值为音频数据列表。
///
/// # 返回值
/// 返回一个 `StaticSoundData` 类型的音频数据。
///
/// # 注意
/// 参数中的音频数据映射即: audio_down 和 audio_up  
/// 都应该包含至少一个以"normal"为键的音频数据键值对，否则会触发 `panic`。
pub async fn key_to_audio(
    key: &KeyEvent,
    audios_down: &HashMap<String, Vec<StaticSoundData>>,
    audios_up: &HashMap<String, Vec<StaticSoundData>>,
) -> StaticSoundData {
    let mut rng = rand::rng();
    let audios = match key.key_type {
        crate::types::key_event::KeyType::Down => audios_down,
        crate::types::key_event::KeyType::Up => audios_up,
    };
    let key_name = &key.key_name;
    match key_name {
        KeyName::Normal(n) => {
            let audio = audios.get(n);
            match audio {
                Some(s) => s.choose(&mut rng).unwrap().clone(),
                None => {
                    let s = audios.get("normal");
                    match s {
                        Some(s) => s.choose(&mut rng).unwrap().clone(),
                        None => {
                            panic!("no audio data,a normal audio file is required")
                        }
                    }
                }
            }
        }
        KeyName::Ctrl => {
            let s = audios.get("ctrl");
            match s {
                Some(s) => s.choose(&mut rng).unwrap().clone(),
                None => {
                    let s = audios.get("normal");
                    match s {
                        Some(s) => s.choose(&mut rng).unwrap().clone(),
                        None => {
                            panic!("no audio data,a normal audio file is required")
                        }
                    }
                }
            }
        }
        KeyName::Alt => {
            let s = audios.get("alt");
            match s {
                Some(s) => s.choose(&mut rng).unwrap().clone(),
                None => {
                    let s = audios.get("normal");
                    match s {
                        Some(s) => s.choose(&mut rng).unwrap().clone(),
                        None => {
                            panic!("no audio data,a normal audio file is required")
                        }
                    }
                }
            }
        }
        KeyName::Shift => {
            let s = audios.get("shift");
            match s {
                Some(s) => s.choose(&mut rng).unwrap().clone(),
                None => {
                    let s = audios.get("normal");
                    match s {
                        Some(s) => s.choose(&mut rng).unwrap().clone(),
                        None => {
                            panic!("no audio data,a normal audio file is required")
                        }
                    }
                }
            }
        }
        KeyName::Tab => {
            let s = audios.get("tab");
            match s {
                Some(s) => s.choose(&mut rng).unwrap().clone(),
                None => {
                    let s = audios.get("normal");
                    match s {
                        Some(s) => s.choose(&mut rng).unwrap().clone(),
                        None => {
                            panic!("no audio data,a normal audio file is required")
                        }
                    }
                }
            }
        }
        KeyName::Enter => {
            let s = audios.get("enter");
            match s {
                Some(s) => s.choose(&mut rng).unwrap().clone(),
                None => {
                    let s = audios.get("normal");
                    match s {
                        Some(s) => s.choose(&mut rng).unwrap().clone(),
                        None => {
                            panic!("no audio data,a normal audio file is required")
                        }
                    }
                }
            }
        }
        KeyName::Delete => {
            let s = audios.get("delete");
            match s {
                Some(s) => s.choose(&mut rng).unwrap().clone(),
                None => {
                    let s = audios.get("normal");
                    match s {
                        Some(s) => s.choose(&mut rng).unwrap().clone(),
                        None => {
                            panic!("no audio data,a normal audio file is required")
                        }
                    }
                }
            }
        }
        KeyName::Esc => {
            let s = audios.get("esc");
            match s {
                Some(s) => s.choose(&mut rng).unwrap().clone(),
                None => {
                    let s = audios.get("normal");
                    match s {
                        Some(s) => s.choose(&mut rng).unwrap().clone(),
                        None => {
                            panic!("no audio data,a normal audio file is required")
                        }
                    }
                }
            }
        }
    }
}
