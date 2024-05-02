use anyhow::Result;
use lazy_static::lazy_static;
use std::collections::VecDeque;
use std::ptr::null_mut;
use std::sync::mpsc::Sender;
use std::sync::{mpsc, Arc, RwLock};
use std::thread::JoinHandle;
use std::{mem, thread};
use winapi::core::PSTR;
use winapi::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use winapi::Win32::System::LibraryLoader::GetModuleHandleA;
use winapi::Win32::System::Threading::Sleep;

use winapi::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, DispatchMessageW, PeekMessageA, SetWindowsHookExA, TranslateMessage,
    UnhookWindowsHookEx, HC_ACTION, HHOOK, KBDLLHOOKSTRUCT, MSG, PM_REMOVE, WH_KEYBOARD_LL,
    WINDOWS_HOOK_ID,
};
use windows as winapi;

use crate::vk::{Action, VK};
//static END_FLAG: AtomicBool = AtomicBool::new(false);

lazy_static! {
    static ref RANK: Arc<RwLock<VecDeque<VK>>> = Arc::new(RwLock::new(VecDeque::new()));
}

pub struct MyWindowsHook {
    thread_id: JoinHandle<()>,
    sender: Sender<bool>,
}
impl MyWindowsHook {
    pub fn new() -> Result<Self> {
        let (sender, recv) = mpsc::channel();

        let thread_id = thread::spawn(move || unsafe {
            let hmod = GetModuleHandleA(PSTR::null()).unwrap();
            let id_hook: WINDOWS_HOOK_ID = WH_KEYBOARD_LL;
            // let a = PSTR(b"window\0".as_ptr() as _);
            let hook = SetWindowsHookExA(id_hook, Some(hook_proc), hmod, 0).unwrap();

            while recv.try_recv().is_err() {
                let ptr_msg: *mut MSG = null_mut();
                let msg = PeekMessageA(ptr_msg, HWND(0), 0, 0, PM_REMOVE);
                if msg == false {
                    Sleep(0);
                } else {
                    let res = TranslateMessage(ptr_msg);
                    if res == false {
                        println!("TranslateMessage res is false");
                    }
                    DispatchMessageW(ptr_msg);
                }
            }
            println!("ending");
            UnhookWindowsHookEx(hook).unwrap();
        });

        Ok(Self { thread_id, sender })
    }
    pub fn end(self) {
        self.sender.send(true).unwrap();
        self.thread_id.join().unwrap();
    }
    /**
     *获取一个vkcode:Ok(Some(vkcode))
     *如果该hooh线程已结束，返回错误
     *如果队列为空，返回Ok(None)
     */
    pub fn try_peek(&self) -> Result<Option<VK>, &'static str> {
        let mut rw = RANK.write().unwrap();
        if rw.is_empty() {
            return Ok(None);
        } else {
            return Ok(Some(rw.pop_front().unwrap()));
        }
    }
}
// 定义钩子处理函数
unsafe extern "system" fn hook_proc(n_code: i32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    if n_code == HC_ACTION as i32 {
        let kb_hook = mem::transmute::<LPARAM, *const KBDLLHOOKSTRUCT>(l_param);
        let vk_code = (*kb_hook).vkCode;
        let flags = (*kb_hook).flags;
        let flags = flags.0;

        let vk = VK::from_cpp(vk_code, flags);
        if let Action::Press = vk.action {
            RANK.write().unwrap().push_back(vk);

            /*
            if flags == 128 {
                println!("Key released: {}", vk_code);
            } else {
                println!("Key pressed: {}", vk_code);
            }
            */
        }
    }

    CallNextHookEx(HHOOK(0), n_code, w_param, l_param)
}
