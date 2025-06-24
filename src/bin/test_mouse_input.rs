use std::env;
use std::path::PathBuf;
use rust_macro::modules::mouse_input::MouseInput;

fn main() {
    // Usage: test_mouse_input [GFCK|GhubMouse]
    let args: Vec<String> = env::args().collect();
    let method = args
        .get(1)
        .map(|s| s.as_str())
        .unwrap_or("GFCK");

    // Update these paths as needed
    let gfck_path = PathBuf::from("lib/GFCK.dll");
    let ghub_path = PathBuf::from("lib/ghub_mouse.dll");

    let mut mouse_input = unsafe {
        match MouseInput::new(gfck_path, ghub_path) {
            Ok(m) => m,
            Err(e) => {
                println!("Failed to initialize MouseInput: {}", e);
                return;
            }
        }
    };

    mouse_input.set_current(method);
    println!("Testing input method: {}", mouse_input.get_current_name());

    // Test actions
    mouse_input.move_relative(100, 0); // Move right
    mouse_input.click(1); // Left click (button code may vary)
    println!("Test actions sent. Waiting 2 seconds...");
    std::thread::sleep(std::time::Duration::from_secs(2));
}
