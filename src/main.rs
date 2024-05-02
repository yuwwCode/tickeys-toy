use std::{thread::sleep, time::Duration};

use my_play::Player;
use my_windows_hook::MyWindowsHook;

mod my_play;
mod my_windows_hook;
mod vk;

fn main() {
    let hook = MyWindowsHook::new().unwrap();

    let mut player = Player::new();
    match player.load_dir_by_json() {
        Ok(_) => (),
        Err(e) => println!("{:?}", e),
    };
    let end_flag = vec![69, 78, 68, 84, 73, 67, 75];
    let mut store = Vec::new();
    let sleep_time = Duration::from_micros(1);
    loop {
        if let Ok(o) = hook.try_peek() {
            if let Some(s) = o {
                // println!("{:?}", s);
                player.play(&s);
                store.push(s.vk_code);
            }
            sleep(sleep_time);
        };
        if store == end_flag {
            break;
        } else if store.len() >= 7 {
            store.remove(0);
        }
    }
    hook.end();
    println!("ended");
}
