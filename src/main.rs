mod modules;

use modules::mouse_input::MouseInput;
use std::{ path::PathBuf, sync::{ Arc, atomic::{ AtomicBool, Ordering } }, thread, time::Duration };
use windows::Win32::UI::Input::KeyboardAndMouse::{ GetAsyncKeyState, VK_F7 };

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let gfck_path = PathBuf::from("lib/GFCK.dll");
    let ghub_path = PathBuf::from("lib/ghub_mouse.dll");

    let mouse_input = unsafe { MouseInput::new(gfck_path, ghub_path)? };

    println!("Current input method: {}", mouse_input.get_current_name());

    let running = Arc::new(AtomicBool::new(false));
    let toggle_state = running.clone();

    thread::spawn(move || {
        let mut prev_state = false;
        loop {
            let pressed = unsafe { (GetAsyncKeyState(VK_F7.0 as i32) & (0x8000u16 as i16)) != 0 };
            if pressed && !prev_state {
                toggle_state.store(!toggle_state.load(Ordering::SeqCst), Ordering::SeqCst);
                println!("Toggled: {}", toggle_state.load(Ordering::SeqCst));
            }
            prev_state = pressed;
            thread::sleep(Duration::from_millis(50));
        }
    });

    while cfg!(windows) {
        if running.load(Ordering::SeqCst) {
            mouse_input.click(1);
            thread::sleep(Duration::from_millis(30));
        } else {
            thread::sleep(Duration::from_millis(10));
        }
    }

    Ok(())
}
