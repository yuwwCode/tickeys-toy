use lazy_static::lazy_static;
use serde::{self, Deserialize, Serialize};
use std::collections::HashMap;
lazy_static! {
    static ref VK_MAP: HashMap<u32, String> = {
        let map: HashMap<u32, String> =
            serde_json::from_reader(std::fs::File::open("./resource/code_kv_map.json").unwrap())
                .unwrap();
        map
    };
}
#[derive(Debug, Serialize, Deserialize)]
pub struct VK {
    pub vk_code: u32,
    pub action: Action,
    pub value: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum Action {
    Press,
    Release,
}

impl VK {
    /*
    action为true:press
     */
    pub fn from_cpp(vk_code: u32, flag: u32) -> Self {
        VK {
            vk_code,
            action: {
                if flag == 128 {
                    Action::Release
                } else {
                    Action::Press
                }
            },
            value: {
                match VK_MAP.get(&vk_code) {
                    Some(s) => Some(s.clone()),
                    None => None,
                }
            },
        }
    }
}
