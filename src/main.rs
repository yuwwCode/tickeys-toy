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
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt, BufReader};

use std::process::Stdio;
use tokio::{process::Command, sync::Mutex};
use types::key_event::KeyEvent;
#[tokio::main]
async fn main() {
    let mut cmd = Command::new("pkexec");
    cmd.arg("./bin/hello");
    use utils::init::{init_audio, init_kira};
    // 创建多个异步任务
    let (manager_result, audios_result) = tokio::join!(init_kira(), init_audio());
    // 处理返回值
    let manager = manager_result.unwrap();
    let (audios_down, audio_up) = audios_result.unwrap();
    //创建当前按下的按钮事件Set
    let mut hoding_key_set: types::hoding_key::HodingKeySet =
        types::hoding_key::HodingKeySet::new();
    // 设置标准输入、标准输出和标准错误输出
    // 将标准输入关闭，避免死锁
    cmd.stdin(Stdio::null());
    // 将标准输出和标准错误输出设置为管道，以便捕获输出
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());
    // 启动子进程
    let mut child = cmd.spawn().expect("Failed to execute command");
    println!("run hello");
    // 获取标准输出和标准错误输出的句柄
    let stdout = child.stdout.take().expect("Failed to open stdout");
    let stderr = child.stderr.take().expect("Failed to open stderr");
    println!("get stdout and stderr");
    // 创建异步的缓冲读取器
    let mut stdout_reader = BufReader::new(stdout).lines();
    let mut stderr_reader = BufReader::new(stderr).lines();

    loop {
        io::stdout().flush().await.unwrap();
        tokio::select! {
            // 读取标准输出
            line = stdout_reader.next_line() => {
                if let Some(line) = line.expect("Failed to read stdout") {
                    play(
                        &line,
                        &mut hoding_key_set,
                        &audios_down,
                        &audio_up,
                        manager.clone(),
                    ).await;
                    println!("Stdout: {}", line);
                } else {
                    // 标准输出结束，跳出循环
                    //break;
                }
            }
            // 读取标准错误输出
            line = stderr_reader.next_line() => {
                if let Some(line) = line.expect("Failed to read stderr") {
                    println!("Stderr: {}", line);
                } else {
                    // 标准错误输出结束，跳出循环
                    break;
                }
            }
        }
    }

    // while flag {
    //     watch(
    //         &mut pubsub,
    //         &mut hoding_key_set,
    //         &audios_down,
    //         &audio_up,
    //         manager.clone(),
    //     )
    //     .await;
    // }
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
async fn play(
    msg: &String,
    hoding_key_set: &mut types::hoding_key::HodingKeySet,
    audios_down: &HashMap<String, Vec<StaticSoundData>>,
    audio_up: &HashMap<String, Vec<StaticSoundData>>,
    manager: Arc<Mutex<AudioManager>>,
) {
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
