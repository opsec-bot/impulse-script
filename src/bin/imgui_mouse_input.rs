use imgui::*;
use std::path::PathBuf;
use rust_macro::modules::mouse_input::MouseInput;

mod support;

fn main() {
    let gfck_path = PathBuf::from("lib/GFCK.dll");
    let ghub_path = PathBuf::from("lib/ghub_mouse.dll");
    let mut mouse_input = unsafe {
        MouseInput::new(gfck_path, ghub_path).expect("Failed to load mouse input DLLs")
    };
    let mut current_method = 0;
    let items = ["GFCK", "GhubMouse"];
    let mut minimized = false;

    // Closure to request window resize
    let _requested_size: Option<[f32; 2]> = None;

    support::simple_init_with_resize(file!(), move |should_run, ui, set_window_size| {
        let window_flags =
            WindowFlags::NO_RESIZE |
            WindowFlags::NO_BRING_TO_FRONT_ON_FOCUS |
            WindowFlags::NO_MOVE |
            WindowFlags::NO_TITLE_BAR;

        if !minimized {
            let size = [320.0, 160.0];
            set_window_size(size);
            ui.window("Mouse Input Method")
                .size(size, Condition::Always)
                .position([0.0, 0.0], Condition::Always) // <-- Force position to (0,0)
                .flags(window_flags)
                .build(|| {
                    ui.text("Mouse Input Method");
                    if ui.combo_simple_string("Input Method", &mut current_method, &items) {
                        mouse_input.set_current(items[current_method]);
                    }
                    if ui.button("Test Click") {
                        mouse_input.click(1);
                    }
                    if ui.button("Move Right") {
                        mouse_input.move_relative(100, 0);
                    }
                    ui.separator();
                    ui.text(format!("Current: {}", mouse_input.get_current_name()));
                    ui.separator();
                    if ui.button("Minimize") {
                        minimized = true;
                    }
                    ui.same_line();
                    if ui.button("Exit") {
                        *should_run = false;
                    }
                });
        } else {
            let size = [120.0, 60.0];
            set_window_size(size);
            ui.window("Minimized")
                .size(size, Condition::Always)
                .position([0.0, 0.0], Condition::Always) // <-- Force position to (0,0)
                .flags(window_flags)
                .build(|| {
                    if ui.button("Restore") {
                        minimized = false;
                    }
                    ui.same_line();
                    if ui.button("Exit") {
                        *should_run = false;
                    }
                });
        }
    });
}
