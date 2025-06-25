mod modules;

use imgui::*;
use modules::input::{ MouseInput, MouseCommand };
use modules::ui::support;
use modules::config::{ Setup, SettingsIO };
use modules::core::{ Control, XmodState };

use std::collections::{ HashMap };
use std::sync::{ Arc, Mutex, mpsc::{ Sender, Receiver, channel } };

fn main() {
    // --- State Initialization ---
    let mut setup = Setup::new(true);
    setup.get_mouse_sensitivity_settings();
    setup.debug_logging();

    let mut settings_io = SettingsIO::new();

    let gfck_path = std::path::PathBuf::from("lib/GFCK.dll");
    let ghub_path = std::path::PathBuf::from("lib/ghub_mouse.dll");
    let mouse_input = Arc::new(
        Mutex::new(unsafe {
            MouseInput::new(gfck_path, ghub_path).expect("Failed to load mouse input DLLs")
        })
    );
    let mut dpi = settings_io.get_dpi();

    // --- Weapon/Hotkey State ---
    let mut all_weapons = settings_io.get_all_wep();
    all_weapons.sort();
    let mut weapon_rpm: HashMap<String, i32> = HashMap::new();

    for weapon in &all_weapons {
        if let Some(rpm) = settings_io.get_weapon_rpm(weapon) {
            weapon_rpm.insert(weapon.clone(), rpm);
        }
    }

    // Remove unused variables
    let mut selected_weapon: Option<String> = None;
    let mut acog_enabled = false;

    let mut add_weapon_popup = false;
    let mut new_weapon_name = String::new();
    let mut new_weapon_rpm = 600;
    let mut new_weapon_class = String::new();
    let weapon_class_options = vec!["AR", "SMG", "LMG", "MP"];
    let mut hotkey_bindings: HashMap<String, String> = HashMap::new();
    let mut hotkey_add_popup = false;
    let mut hotkey_weapon = String::new();
    let mut hotkey_key = String::new();
    let mut exit_hotkey = settings_io
        .get_profile_hotkey("exit")
        .unwrap_or_else(|| "None".to_string());
    let mut mouse_method = match settings_io.settings.get("MOUSE", "method").as_deref() {
        Some("GhubMouse") => 1,
        _ => 0,
    };

    // --- Settings State ---
    let mut fov = setup.get_fov() as i32;
    let mut sens = setup.get_sensitivity() as i32;
    let mut sens_1x = setup.get_sensitivity_modifier_1() as i32;
    let mut sens_25x = setup.get_sensitivity_modifier_25() as i32;

    // --- Mouse Command Channel ---
    let (tx, rx): (Sender<MouseCommand>, Receiver<MouseCommand>) = channel();

    // --- Control Handler State ---
    let mut control = Control::new();
    control.set_sender(tx);
    control.run_threaded();

    // --- ImGui Main Loop ---
    let mut xmod_state = XmodState { x_flip: 1, x_once_done: false };
    let mut prev_weapon: Option<String> = None;
    let mut prev_acog = false;
    
    support::simple_init_with_resize(file!(), move |_should_run, ui, set_window_size| {
        let window_flags =
            WindowFlags::NO_RESIZE |
            WindowFlags::NO_BRING_TO_FRONT_ON_FOCUS |
            WindowFlags::NO_MOVE |
            WindowFlags::NO_TITLE_BAR;

        let size = [600.0, 420.0];
        set_window_size(size);
        ui.window("RCS Config")
            .size(size, Condition::Always)
            .position([0.0, 0.0], Condition::Always)
            .flags(window_flags)
            .build(|| {
                // Handle scheduled mouse commands on the main thread
                while let Ok(cmd) = rx.try_recv() {
                    match cmd {
                        MouseCommand::Move(mut x, y) => {
                            // Only run Xmod logic and move mouse if a weapon is selected
                            if let Some(selected) = selected_weapon.as_ref() {
                                // Get xmod value directly from settings_io instead of HashMap
                                let (_, _, xmod_val) = settings_io.get_weapon_values(selected, acog_enabled);
                                match xmod_val as i32 {
                                    -1 => {
                                        x *= xmod_state.x_flip;
                                        xmod_state.x_flip *= -1;
                                    }
                                    0 => {
                                        if xmod_state.x_once_done {
                                            x = 0;
                                        } else {
                                            xmod_state.x_once_done = true;
                                        }
                                    }
                                    1 => {
                                        // No change
                                    }
                                    _ => {
                                        x = ((x as f32) * xmod_val) as i32;
                                    }
                                }
                                mouse_input.lock().unwrap().move_relative(x, y);
                            } else {
                                // No weapon selected: do not move mouse!
                                // Optionally, log or ignore
                            }
                        }
                        MouseCommand::Click(b) => mouse_input.lock().unwrap().click(b),
                        MouseCommand::Down(b) => mouse_input.lock().unwrap().down(b),
                        MouseCommand::Up(b) => mouse_input.lock().unwrap().up(b),
                    }
                }

                if let Some(_tab_bar_token) = ui.tab_bar("main_tabs") {
                    // --- Recoil Control Tab ---
                    if let Some(_tab_item_token) = ui.tab_item("Recoil Control") {
                        let acog_label = String::from("ACOG");
                        ui.checkbox(&acog_label, &mut acog_enabled);
                        ui.same_line();

                        // Weapon dropdown
                        let weapons_by_class = settings_io.get_weapons_by_class();
                        let combo_width = 200.0;
                        ui.set_next_item_width(combo_width);
                        if
                            let Some(_combo_token) = ui.begin_combo(
                                "Select Weapon",
                                selected_weapon.as_deref().unwrap_or("Select...")
                            )
                        {
                            for class in &weapon_class_options {
                                if let Some(weapons) = weapons_by_class.get(*class) {
                                    ui.text(format!("--- {} ---", class));
                                    for weapon in weapons {
                                        if
                                            ui
                                                .selectable_config(weapon)
                                                .selected(
                                                    selected_weapon.as_deref() == Some(weapon)
                                                )
                                                .build()
                                        {
                                            selected_weapon = Some(weapon.clone());
                                        }
                                    }
                                }
                            }
                        }

                        ui.spacing();
                        let dropdown_x = ui.cursor_pos()[0];
                        let dropdown_y = ui.cursor_pos()[1];

                        ui.set_cursor_pos([dropdown_x, dropdown_y + 30.0]);

                        let button_width = ui.calc_text_size("Add Weapon")[0] + 32.0;
                        let button_x = dropdown_x + combo_width - button_width + 83.0;
                        let button_y = dropdown_y + 30.0 - 33.0;

                        ui.set_cursor_pos([button_x, button_y]);
                        if ui.button("Add Weapon") {
                            add_weapon_popup = true;
                        }

                        // X/Y Sliders for selected weapon
                        if let Some(weapon) = &selected_weapon {
                            let (x, y, xmod_val) = settings_io.get_weapon_values(
                                weapon,
                                acog_enabled
                            );

                            if prev_weapon != Some(weapon.clone()) || prev_acog != acog_enabled {
                                xmod_state.x_flip = 1;
                                xmod_state.x_once_done = false;
                                prev_weapon = Some(weapon.clone());
                                prev_acog = acog_enabled;

                                let rpm = weapon_rpm.get(weapon).copied().unwrap_or(600) as f32;
                                let timing = (4234.44 / rpm + 2.58).round() as i32;
                                control.update(x as i32, y as i32, timing, xmod_val);
                                let _ = control.current(true);
                            }

                            // Use settings_io to load values
                            let (mut x, mut y, mut xmod_val) = settings_io.get_weapon_values(
                                weapon,
                                acog_enabled
                            );

                            let mut changed = false;
                            let mut x_int = x.round() as i32;
                            let mut y_int = y.round() as i32;
                            let mut xmod_int = xmod_val.round() as i32;
                            changed |= ui.slider_config("X", -2, 2).build(&mut x_int);
                            changed |= ui.slider_config("Y", 1, 10).build(&mut y_int);
                            changed |= ui.slider_config("Xmod", -1, 2).build(&mut xmod_int);
                            x = x_int as f32;
                            y = y_int as f32;
                            xmod_val = xmod_int as f32;

                            if changed {
                                settings_io.save_weapon_values(
                                    weapon,
                                    x,
                                    y,
                                    xmod_val,
                                    acog_enabled
                                );
                                let rpm = weapon_rpm.get(weapon).copied().unwrap_or(600) as f32;
                                let timing = (4234.44 / rpm + 2.58).round() as i32;
                                control.update(x as i32, y as i32, timing, xmod_val);
                                let _ = control.current(true);
                            }
                        }

                        if add_weapon_popup {
                            ui.open_popup("AddWeaponPopup");
                        }
                        if let Some(_popup_token) = ui.begin_popup("AddWeaponPopup") {
                            ui.input_text("Gun Name", &mut new_weapon_name).build();
                            ui.input_int("RPM", &mut new_weapon_rpm).build();
                            if
                                let Some(_combo_token) = ui.begin_combo(
                                    "Weapon Class",
                                    new_weapon_class.as_str()
                                )
                            {
                                for class in &weapon_class_options {
                                    if
                                        ui
                                            .selectable_config(class)
                                            .selected(&new_weapon_class == *class)
                                            .build()
                                    {
                                        new_weapon_class = (*class).to_string();
                                    }
                                }
                            }
                            if ui.button("Add") {
                                if !new_weapon_name.is_empty() && !new_weapon_class.is_empty() {
                                    settings_io.settings.update(&new_weapon_name, "X", 0.0);
                                    settings_io.settings.update(&new_weapon_name, "Y", 1.0);
                                    settings_io.settings.update(&new_weapon_name, "xmod", 0.0);
                                    settings_io.settings.update(
                                        &new_weapon_name,
                                        "RPM",
                                        new_weapon_rpm
                                    );
                                    settings_io.settings.update(
                                        &new_weapon_name,
                                        "class",
                                        &new_weapon_class
                                    );
                                    settings_io.settings.write();

                                    weapon_rpm.insert(new_weapon_name.clone(), new_weapon_rpm);
                                    all_weapons.push(new_weapon_name.clone());
                                    selected_weapon = Some(new_weapon_name.clone());
                                    new_weapon_name.clear();
                                    new_weapon_class.clear();
                                    new_weapon_rpm = 600;
                                    add_weapon_popup = false;
                                    ui.close_current_popup();
                                }
                            }
                            if ui.button("Cancel") {
                                add_weapon_popup = false;
                                ui.close_current_popup();
                            }
                        }
                    }

                    // --- Hotkeys Tab ---
                    if let Some(_tab_item_token) = ui.tab_item("Hotkeys") {
                        // Exit Button Hotkey
                        ui.text("Exit Button Hotkey:");
                        static mut CAPTURING_EXIT: bool = false;
                        let mut capturing_exit = unsafe { CAPTURING_EXIT };
                        if ui.button(&format!("Current: {}", exit_hotkey)) {
                            capturing_exit = true;
                        }
                        if capturing_exit {
                            ui.text("Press a key (ESC to clear)...");
                            if
                                let Some((imgui_key, _)) = ui
                                    .io()
                                    .keys_down.iter()
                                    .enumerate()
                                    .find(|&(_, &down)| down)
                            {
                                if imgui_key == (imgui::Key::Escape as usize) {
                                    exit_hotkey = "None".to_string();
                                } else {
                                    exit_hotkey = modules::ui::keybinds
                                        ::imgui_key_to_name(imgui_key as u32)
                                        .to_string();
                                }
                                settings_io.save_profile_hotkey("exit", &exit_hotkey);
                                capturing_exit = false;
                            }
                        }
                        unsafe {
                            CAPTURING_EXIT = capturing_exit;
                        }

                        // Add Hotkey Binding
                        if ui.button("+ Add Hotkey Binding") {
                            hotkey_add_popup = true;
                        }
                        if hotkey_add_popup {
                            ui.open_popup("AddHotkeyPopup");
                        }
                        if let Some(_popup_token) = ui.begin_popup("AddHotkeyPopup") {
                            // Weapon dropdown
                            if
                                let Some(_combo_token) = ui.begin_combo(
                                    "Weapon",
                                    hotkey_weapon.as_str()
                                )
                            {
                                for weapon in &all_weapons {
                                    if
                                        ui
                                            .selectable_config(weapon)
                                            .selected(&hotkey_weapon == weapon)
                                            .build()
                                    {
                                        hotkey_weapon = weapon.clone();
                                    }
                                }
                            }
                            // Key capture for hotkey
                            static mut CAPTURING_HOTKEY: bool = false;
                            let mut capturing_hotkey = unsafe { CAPTURING_HOTKEY };
                            if ui.button("Capture Key") {
                                capturing_hotkey = true;
                            }
                            if capturing_hotkey {
                                ui.text("Press a key...");
                                if
                                    let Some((imgui_key, _)) = ui
                                        .io()
                                        .keys_down.iter()
                                        .enumerate()
                                        .find(|&(_, &down)| down)
                                {
                                    hotkey_key = modules::ui::keybinds
                                        ::imgui_key_to_name(imgui_key as u32)
                                        .to_string();
                                    capturing_hotkey = false;
                                }
                            }
                            unsafe {
                                CAPTURING_HOTKEY = capturing_hotkey;
                            }
                            ui.input_text("Key", &mut hotkey_key).build();
                            if ui.button("Bind") {
                                if !hotkey_weapon.is_empty() && !hotkey_key.is_empty() {
                                    settings_io.save_profile_hotkey(&hotkey_weapon, &hotkey_key);
                                    hotkey_bindings.insert(
                                        hotkey_key.clone(),
                                        hotkey_weapon.clone()
                                    );
                                    hotkey_add_popup = false;
                                    ui.close_current_popup();
                                }
                            }
                            if ui.button("Cancel") {
                                hotkey_add_popup = false;
                                ui.close_current_popup();
                            }
                        }
                        // List current hotkey bindings
                        ui.separator();
                        ui.text("Current Hotkey Bindings:");
                        for (key, weapon) in &hotkey_bindings {
                            ui.text(format!("{} -> {}", key, weapon));
                        }
                    }

                    // --- Mouse Tab ---
                    if let Some(_tab_item_token) = ui.tab_item("Mouse") {
                        ui.text("Mouse Input Method:");
                        let mut method = mouse_method;
                        if ui.radio_button("gfck", &mut method, 0) {
                        }
                        ui.same_line();
                        if ui.radio_button("ghubmouse", &mut method, 1) {
                        }
                        if method != mouse_method {
                            mouse_input
                                .lock()
                                .unwrap()
                                .set_current(if method == 0 { "GFCK" } else { "GhubMouse" });
                            settings_io.settings.update("MOUSE", "method", if method == 0 {
                                "GFCK"
                            } else {
                                "GhubMouse"
                            });
                            settings_io.settings.write();
                            println!(
                                "Switched mouse input method to: {}",
                                mouse_input.lock().unwrap().get_current_name()
                            );
                            mouse_method = method;
                        }
                        // // Test buttons for mouse input
                        // if ui.button("Test Click") {
                        //     mouse_input.lock().unwrap().click(1);
                        // }
                        // if ui.button("Move Right") {
                        //     mouse_input.lock().unwrap().move_relative(100, 0);
                        // }
                    }

                    // --- Settings Tab ---
                    if let Some(_tab_item_token) = ui.tab_item("Settings") {
                        if ui.button("Auto-import from GameSettings.ini") {
                            setup.get_mouse_sensitivity_settings();
                            fov = setup.get_fov() as i32;
                            sens = setup.get_sensitivity() as i32;
                            sens_1x = setup.get_sensitivity_modifier_1() as i32;
                            sens_25x = setup.get_sensitivity_modifier_25() as i32;
                            settings_io.settings.update("GAME", "fov", fov);
                            settings_io.settings.update("GAME", "sens", sens);
                            settings_io.settings.update("GAME", "sens_1x", sens_1x);
                            settings_io.settings.update("GAME", "sens_25x", sens_25x);
                        }
                        ui.separator();
                        if ui.input_int("DPI", &mut dpi).build() {
                            settings_io.set_dpi(dpi);
                        }
                        if ui.slider_config("FOV", 60, 90).build(&mut fov) {
                            settings_io.settings.update("GAME", "fov", fov);
                            settings_io.settings.write();
                        }
                        if ui.slider_config("Sensitivity", 1, 100).build(&mut sens) {
                            settings_io.settings.update("GAME", "sens", sens);
                            settings_io.settings.write();
                        }
                        if ui.slider_config("1x Sensitivity", 1, 100).build(&mut sens_1x) {
                            settings_io.settings.update("GAME", "sens_1x", sens_1x);
                            settings_io.settings.write();
                        }
                        if ui.slider_config("2.5x Sensitivity", 1, 100).build(&mut sens_25x) {
                            settings_io.settings.update("GAME", "sens_25x", sens_25x);
                            settings_io.settings.write();
                        }
                    }
                }
            });
    });
}