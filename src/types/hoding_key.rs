use std::collections::{HashMap, HashSet};

use super::key_event::{KeyEvent, KeyName, KeyType};
/// A set used to store key events.  
/// This struct contains a `HashMap` that stores the mapping relationship of key events.  
/// The key of the key event is of type `KeyName`, and the value is of type `KeyEvent`.  
/// When a key is pressed, the key event is added to the set; when a key is released, the key event is removed from the set.  
/// This set can be used to detect the state of a key, such as determining whether a key is pressed.  
/// 用于存储按键事件的集合  
/// 该结构体包含一个 `HashMap`，用于存储按键事件的映射关系。  
/// 按键事件的键为 `KeyName` 类型，值为 `KeyEvent` 类型。  
/// 当按键已经按下时，会将按键事件添加到集合中；当按键释放时，会从集合中移除按键事件。  
/// 该集合可以用于检测按键的状态，例如判断某个按键是否被按下。  
pub struct HodingKeySet {
    pub key_map: HashMap<KeyName, KeyEvent>,
}
impl HodingKeySet {
    /// Create a new instance of HodingKeySet.
    /// 创建一个新的 HodingKeySet 实例。
    pub fn new() -> Self {
        Self {
            key_map: HashMap::new(),
        }
    }

    /// Add a key to the key_map.
    /// ## false indicates that the audio should not be played.
    /// ## true indicates that the audio should be played.
    /// ### If the key is of the press type
    /// If the key already exists, return false.
    /// If the key does not exist, add the key and return true.
    /// ### If the key is of the release type
    /// If the key already exists, remove the key and return true.
    /// If the key does not exist, return true.
    /// 向key_map中添加key
    /// ## false表示该音频不应该被播放
    /// ## true表示该音频应该被播放
    /// ### 如果key为按下类型
    /// 如果key已经存在，返回false  
    /// 如果key不存在，则添加key，返回true
    /// ### 如果key为释放类型
    /// 则删除key，返回true  

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
            self.key_map.remove(key_name);
            return true;
        }
    }
}
