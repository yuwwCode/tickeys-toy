use anyhow::Result;
use kira::{
    sound::static_sound::StaticSoundData, AudioManager, AudioManagerSettings, DefaultBackend,
};
use redis::{self, Commands};
use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex;
use types::key_event::KeyEvent;

use crate::types;
///
/// 初始化kira，并返回一个Arc<Mutex<AudioManager>>
/// 一个可以在多个线程中共享的音频管理器
pub async fn init_kira() -> Result<Arc<Mutex<AudioManager>>> {
    let manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())?;
    let manager = Arc::new(Mutex::new(manager));
    Ok(manager)
}
///inialaize the audio,by reading audio files from the default 'assets/sounds' folder.  
///  Returns a tuple where the first element is a mapping of audio data corresponding to key presses,  
/// and the second element is a mapping of audio data corresponding to key releases.  
/// Each audio data mapping has a key of the button name and a value of a `Vec` of audio data.  
///初始化audio，从默认的assets/sounds文件夹中读取音频文件  
/// 返回一个元组，第一个元素：按下按键时对应的音频数据映射，第二个元素：释放按键时对应的音频数据映射  
/// 每个音频数据映射的键为按键名称，值为音频数据列表
pub async fn init_audio() -> Result<(
    HashMap<String, Vec<StaticSoundData>>,
    HashMap<String, Vec<StaticSoundData>>,
)> {
    let mut mp3_files = Vec::new();
    let entries = std::fs::read_dir("./assets/sounds")?;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            mp3_files.push(path);
        }
    }
    for i in mp3_files.iter() {
        println!("{:?}", i);
    }
    let mut audios_down: HashMap<String, Vec<StaticSoundData>> = HashMap::new();
    let mut audios_up: HashMap<String, Vec<StaticSoundData>> = HashMap::new();
    for i in mp3_files.iter() {
        let data = StaticSoundData::from_file(i)?;
        let file_name = match &i.file_name() {
            Some(s) => s.to_string_lossy(),
            None => return Err(anyhow::Error::msg("error in read audio file")),
        };
        let file_name = file_name.to_string();
        let parts: Vec<&str> = file_name.split('.').collect();
        let parts: Vec<&str> = parts[0].split('_').collect();
        if parts.len() < 2 {
            return Err(anyhow::Error::msg(format!(
                "error in read audio file,need file name like: key_down.mp3,error file is {:?}",
                file_name,
            )));
        }
        let audio_name = parts[0].to_string();
        let audio_type = parts[1].to_string();
        let audios = match audio_type.as_str() {
            "down" => &mut audios_down,
            "up" => &mut audios_up,
            _ => {
                continue;
            }
        };
        match audios.get_mut(&audio_name) {
            Some(audio) => audio.push(data),
            None => {
                audios.insert(audio_name, vec![data]);
            }
        }
    }
    println!("audios_down:");
    for i in audios_down.iter() {
        println!("{:?}", i.0);
    }
    println!("audios_up:");
    for i in audios_up.iter() {
        println!("{:?}", i.0);
    }
    Ok((audios_down, audios_up))
}
