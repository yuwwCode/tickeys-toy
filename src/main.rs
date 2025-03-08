// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod types;
mod utils;
use anyhow::Result;
use kira::{
    sound::static_sound::StaticSoundData, AudioManager, AudioManagerSettings, DefaultBackend,
};
use redis::{self, Commands};
use std::{collections::HashMap, sync::Arc};

use tokio::sync::Mutex;
use types::key_event::KeyEvent;
#[tokio::main]
async fn main() {
    //设定redis连接
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_connection().unwrap();
    con.set::<&str, &str, ()>("rs_to_py_flag", "ready").unwrap();
    // 发布消息给python
    let _ = con
        .publish::<&str, &str, ()>("py_to_rs_channel", "")
        .unwrap();
    // 订阅redis频道
    let mut pubsub: redis::PubSub<'_> = con.as_pubsub();
    pubsub.subscribe("py_to_rs_channel").unwrap();

    let flag = true;
    use utils::init::{init_audio, init_kira};
    // 创建多个异步任务
    let (manager_result, audios_result) = tokio::join!(init_kira(), init_audio());
    // 处理返回值
    let manager = manager_result.unwrap();
    let (audios_down, audio_up) = audios_result.unwrap();
    //创建当前按下的按钮事件Set
    let mut hoding_key_set: types::hoding_key::HodingKeySet =
        types::hoding_key::HodingKeySet::new();
    while flag {
        watch(
            &mut pubsub,
            &mut hoding_key_set,
            &audios_down,
            &audio_up,
            manager.clone(),
        )
        .await;
    }
    tickeys_lib::run();
}
async fn play_data(data: StaticSoundData, manager: Arc<Mutex<AudioManager>>) {
    let res = manager.lock().await.play(data);
    match res {
        Ok(_) => {}
        Err(e) => {
            println!("{:?}", e);
            panic!("error in play data");
        }
    }
}
async fn watch(
    pubsub: &mut redis::PubSub<'_>,
    hoding_key_set: &mut types::hoding_key::HodingKeySet,
    audios_down: &HashMap<String, Vec<StaticSoundData>>,
    audio_up: &HashMap<String, Vec<StaticSoundData>>,
    manager: Arc<Mutex<AudioManager>>,
) {
    if let Ok(msg) = pubsub.get_message() {
        let msg: String = msg.get_payload().unwrap();
        let key_event: KeyEvent = KeyEvent::from_str(&msg).unwrap();
        let res = hoding_key_set.add_key(&key_event);
        if !res {
            return;
        }
        let data = utils::key_to_audio::key_to_audio(&key_event, &audios_down, &audio_up).await;
        let manager_clone = manager.clone();
        tokio::spawn(async move {
            play_data(data, manager_clone).await;
        });
    }
}
