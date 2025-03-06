use std::collections::HashMap;

use kira::sound::static_sound::StaticSoundData;

use crate::types::key_event::{KeyEvent, KeyName};
use rand::{self, seq::IndexedRandom};
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
