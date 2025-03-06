use std::collections::{HashMap, HashSet};

use super::key_event::{KeyEvent, KeyName, KeyType};

pub struct HodingKeySet {
    pub key_map: HashMap<KeyName, KeyEvent>,
}
impl HodingKeySet {
    pub fn new() -> Self {
        Self {
            key_map: HashMap::new(),
        }
    }

    /*
     * 向key_map中添加key
     * 如果key已经存在，返回false，
     * 如果key不存在，则添加key，返回true
     */
    pub fn add_key(&mut self, key: &KeyEvent) -> bool {
        let key_name = &key.key_name;
        let is_down = key.key_type == KeyType::Down;
        if is_down {
            match self.key_map.get(key_name) {
                Some(_) => {
                    return false;
                }
                None => {
                    self.key_map.insert(key_name.clone(), key.clone());
                    return true;
                }
            }
        } else {
            match self.key_map.get(key_name) {
                Some(_) => {
                    self.key_map.remove(key_name);
                    return true;
                }
                None => return true,
            }
        }
    }
}
