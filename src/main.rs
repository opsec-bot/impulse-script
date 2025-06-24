use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

use windows::Win32::UI::Input::KeyboardAndMouse::*;
use windows::Win32::UI::Input::KeyboardAndMouse::SendInput;
use windows::Win32::UI::Input::KeyboardAndMouse::INPUT;
use windows::Win32::UI::Input::KeyboardAndMouse::INPUT_KEYBOARD;
use windows::Win32::UI::Input::KeyboardAndMouse::KEYBDINPUT;
use windows::Win32::UI::Input::KeyboardAndMouse::KEYBD_EVENT_FLAGS;
use windows::Win32::UI::Input::KeyboardAndMouse::KEYEVENTF_KEYUP;
use windows::Win32::UI::Input::KeyboardAndMouse::VIRTUAL_KEY;

// Function to send a key press and release
fn send_key(scan_code: u16) {
    unsafe {
        let mut input_down = INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: std::mem::zeroed(),
        };
        input_down.Anonymous.ki = KEYBDINPUT {
            wVk: VIRTUAL_KEY(0),
            wScan: scan_code,
            dwFlags: KEYBD_EVENT_FLAGS(KEYEVENTF_SCANCODE.0),
            time: 0,
            dwExtraInfo: 0,
        };

        let mut input_up = INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: std::mem::zeroed(),
        };
        input_up.Anonymous.ki = KEYBDINPUT {
            wVk: VIRTUAL_KEY(0),
            wScan: scan_code,
            dwFlags: KEYBD_EVENT_FLAGS(KEYEVENTF_SCANCODE.0 | KEYEVENTF_KEYUP.0),
            time: 0,
            dwExtraInfo: 0,
        };

        let mut inputs = [input_down, input_up];
        SendInput(&mut inputs, std::mem::size_of::<INPUT>() as i32);
    }
}

fn main() {
    let running = Arc::new(AtomicBool::new(false));
    let running_clone = running.clone();

    // Toggle on F7
    thread::spawn(move || {
        let mut prev_state = false;
        loop {
            let pressed = unsafe { GetAsyncKeyState(VK_F7.0 as i32) & (0x8000u16 as i16) != 0 };
            if pressed && !prev_state {
                let current = running_clone.load(Ordering::SeqCst);
                running_clone.store(!current, Ordering::SeqCst);
                println!("Toggled: {}", !current);
            }
            prev_state = pressed;
            thread::sleep(Duration::from_millis(50));
        }
    });

    // Spam "E" while active
    loop {
        if running.load(Ordering::SeqCst) {
           send_key(0x12); // 'E' key scan code
            thread::sleep(Duration::from_millis(30));
        } else {
            thread::sleep(Duration::from_millis(10));
        }
    }
}
