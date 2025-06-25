mod modules;

use imgui::*;
use modules::mouse_input::MouseInput;
use modules::support;
use modules::handlers::{ setup_class::Setup, settings_io::SettingsIO };
use modules::handlers::control::Control;
use modules::mouse_command::MouseCommand;

use std::collections::{ HashMap, BTreeMap };
use std::sync::{ Arc, Mutex, mpsc::{ Sender, Receiver, channel } };

#[derive(PartialEq, Eq, Clone, Copy)]
enum Tab {
    RecoilControl,
    Hotkeys,
    Mouse,
    Settings,
}

fn main() {
    // --- State Initialization ---
    let mut setup = Setup::new(true);
    setup.get_mouse_sensitivity_settings();
    setup.create_config_file();
    setup.debug_logging();

    let mut settings_io = SettingsIO::new();

    let gfck_path = std::path::PathBuf::from("lib/GFCK.dll");
    let ghub_path = std::path::PathBuf::from("lib/ghub_mouse.dll");
    let mouse_input = Arc::new(
        Mutex::new(unsafe {
            MouseInput::new(gfck_path, ghub_path).expect("Failed to load mouse input DLLs")
        })
    );

    // --- Weapon/Hotkey State ---
    let mut weapon_classes: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut weapon_to_class: HashMap<String, String> = HashMap::new();
    let mut weapon_rpm: HashMap<String, i32> = HashMap::new();
    let mut weapon_xy: HashMap<String, (f32, f32)> = HashMap::new();
    let mut weapon_xmod: HashMap<String, f32> = HashMap::new();
    let mut weapon_xy_acog: HashMap<String, (f32, f32)> = HashMap::new();
    let mut weapon_xmod_acog: HashMap<String, f32> = HashMap::new();
    let mut all_weapons: Vec<String> = vec![];
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
    let mut mouse_method = 0; // 0 = gfck, 1 = ghubmouse

    // --- Settings State ---
    let mut fov = setup.get_fov() as i32;
    let mut sens = setup.get_sensitivity() as i32;
    let mut sens_1x = setup.get_sensitivity_modifier_1() as i32;
    let mut sens_25x = setup.get_sensitivity_modifier_25() as i32;
    let mut xmod = setup.get_x_factor();
    let mut dpi = setup.get_dpi(); // Use the public getter method

    // --- Mouse Command Channel ---
    let (tx, rx): (Sender<MouseCommand>, Receiver<MouseCommand>) = channel();

    // --- Control Handler State ---
    let mut control = Control::new(Arc::clone(&mouse_input));
    control.set_sender(tx.clone());
    control.run_threaded();

    // --- Parse hardcoded weapons from config ---
    let config = &settings_io.settings;
    let mut add_weapon = |class: &str, timings_key: &str| {
        if let Some(timings_str) = config.get("RCS", timings_key) {
            let timings_str = timings_str.replace('\'', "\"");
            if let Ok(map) = serde_json::from_str::<HashMap<String, i32>>(&timings_str) {
                for (weapon, rpm) in map {
                    weapon_classes.entry(class.to_string()).or_default().push(weapon.clone());
                    weapon_to_class.insert(weapon.clone(), class.to_string());
                    weapon_rpm.insert(weapon.clone(), rpm);
                    all_weapons.push(weapon.clone());
                    // Load X/Y/Xmod and X/Y/Xmod_acog if present
                    let x = config
                        .get(&weapon, "X")
                        .and_then(|v| v.parse().ok())
                        .unwrap_or(0.0);
                    let y = config
                        .get(&weapon, "Y")
                        .and_then(|v| v.parse().ok())
                        .unwrap_or(1.0);
                    let xmod = config
                        .get(&weapon, "Xmod")
                        .and_then(|v| v.parse().ok())
                        .unwrap_or(1.0);
                    weapon_xy.insert(weapon.clone(), (x, y));
                    weapon_xmod.insert(weapon.clone(), xmod);
                    let x_acog = config
                        .get(&weapon, "X_acog")
                        .and_then(|v| v.parse().ok())
                        .unwrap_or(x);
                    let y_acog = config
                        .get(&weapon, "Y_acog")
                        .and_then(|v| v.parse().ok())
                        .unwrap_or(y);
                    let xmod_acog = config
                        .get(&weapon, "Xmod_acog")
                        .and_then(|v| v.parse().ok())
                        .unwrap_or(xmod);
                    weapon_xy_acog.insert(weapon.clone(), (x_acog, y_acog));
                    weapon_xmod_acog.insert(weapon.clone(), xmod_acog);
                }
            }
        }
    };
    add_weapon("AR", "ar_timings");
    add_weapon("SMG", "smg_timings");
    add_weapon("LMG", "lmg_timings");
    add_weapon("MP", "mp_timings");
    all_weapons.sort();

    // --- Calculate X/Y for each weapon using calculator handler ---
    use modules::handlers::calculator::ScopeSensitivityCalculator;
    let mut calc = ScopeSensitivityCalculator::new();
    for weapon in &all_weapons {
        let class = weapon_to_class
            .get(weapon)
            .map(|s| s.as_str())
            .unwrap_or("AR");
        let rpm = weapon_rpm.get(weapon).copied().unwrap_or(600);
        // Calculate the correct timing (interval between shots) in milliseconds
        // RPM = rounds per minute, so interval_ms = 60000 / RPM
        let interval_ms = if rpm > 0 { 60000.0 / rpm as f32 } else { 100.0 };
        let (x, y) = match class {
            "AR" | "SMG" | "LMG" | "MP" => {
                let rcs_vals = calc.get_rcs_values(
                    setup.get_fov() as f64,
                    setup.get_sensitivity() as f64,
                    setup.get_sensitivity_modifier_1() as f64,
                    setup.get_sensitivity_modifier_25() as f64,
                    setup.get_x_factor() as f64
                );
                let x_val = rcs_vals.get(0).copied().unwrap_or(0) as f32;
                let y_val = 1.0; // You can adjust this as needed for vertical compensation
                (x_val, y_val)
            }
            _ => (0.0, 0.0),
        };
        // Store the interval_ms as the "Y" value for timing (or use it directly in control.update)
        weapon_xy.insert(weapon.clone(), (x, interval_ms));
    }

    // --- ImGui Main Loop ---
    // Use the same trick to "hide" the window behind the ImGui window by always matching the window size and position.
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
                        MouseCommand::Move(x, y) => mouse_input.lock().unwrap().move_relative(x, y),
                        MouseCommand::Click(b) => mouse_input.lock().unwrap().click(b),
                        MouseCommand::Down(b) => mouse_input.lock().unwrap().down(b),
                        MouseCommand::Up(b) => mouse_input.lock().unwrap().up(b),
                    }
                }

                if let Some(_tab_bar_token) = ui.tab_bar("main_tabs") {
                    // --- Recoil Control Tab ---
                    if let Some(_tab_item_token) = ui.tab_item("Recoil Control") {
                        // Weapon dropdown (searchable if possible)
                        if
                            let Some(_combo_token) = ui.begin_combo(
                                "Select Weapon",
                                selected_weapon.as_deref().unwrap_or("Select...")
                            )
                        {
                            for class in &weapon_class_options {
                                if let Some(weapons) = weapon_classes.get(*class) {
                                    ui.text(format!("-- {} --", class));
                                    for weapon in weapons {
                                        let is_selected =
                                            selected_weapon.as_deref() == Some(weapon.as_str());
                                        if
                                            ui
                                                .selectable_config(weapon)
                                                .selected(is_selected)
                                                .build()
                                        {
                                            selected_weapon = Some(weapon.clone());
                                        }
                                    }
                                }
                            }
                        }
                        // Acog toggle
                        if ui.checkbox("Acog (2.5x)", &mut acog_enabled) {
                            // No-op, state is toggled
                        }
                        // X/Y Sliders for selected weapon (default or acog)
                        if let Some(weapon) = &selected_weapon {
                            let (mut x, mut y) = if acog_enabled {
                                weapon_xy_acog.get(weapon).copied().unwrap_or((0.0, 1.0))
                            } else {
                                weapon_xy.get(weapon).copied().unwrap_or((0.0, 1.0))
                            };
                            let mut xmod_val = if acog_enabled {
                                weapon_xmod_acog.get(weapon).copied().unwrap_or(0.02)
                            } else {
                                weapon_xmod.get(weapon).copied().unwrap_or(0.02)
                            };
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
                                if acog_enabled {
                                    weapon_xy_acog.insert(weapon.clone(), (x, y));
                                    weapon_xmod_acog.insert(weapon.clone(), xmod_val);
                                    settings_io.settings.update(weapon, "X_acog", x);
                                    settings_io.settings.update(weapon, "Y_acog", y);
                                    settings_io.settings.update(weapon, "Xmod_acog", xmod_val);
                                    // Use control for acog profile as well
                                    control.update(x as i32, y as i32, y as i32, xmod_val); // y as i32 is interval_ms
                                } else {
                                    weapon_xy.insert(weapon.clone(), (x, y));
                                    weapon_xmod.insert(weapon.clone(), xmod_val);
                                    settings_io.settings.update(weapon, "X", x);
                                    settings_io.settings.update(weapon, "Y", y);
                                    settings_io.settings.update(weapon, "Xmod", xmod_val);
                                    // Use control for default profile
                                    control.update(x as i32, y as i32, y as i32, xmod_val); // y as i32 is interval_ms
                                }
                                settings_io.settings.write();
                                // Show current values for debug
                                let _ = control.current(true);
                            }
                        }

                        // Add Weapon Dialog
                        if ui.button("Add Weapon") {
                            add_weapon_popup = true;
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
                                    // Save new weapon to config
                                    settings_io.settings.update(
                                        &new_weapon_name,
                                        "class",
                                        &new_weapon_class
                                    );
                                    settings_io.settings.update(
                                        &new_weapon_name,
                                        "rpm",
                                        new_weapon_rpm
                                    );
                                    settings_io.settings.write();
                                    weapon_classes
                                        .entry(new_weapon_class.clone())
                                        .or_default()
                                        .push(new_weapon_name.clone());
                                    weapon_to_class.insert(
                                        new_weapon_name.clone(),
                                        new_weapon_class.clone()
                                    );
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
                                let Some(key) = ui
                                    .io()
                                    .keys_down.iter()
                                    .position(|&down| down)
                            {
                                // Map key index to string as needed
                                if key == (imgui::Key::Escape as usize) {
                                    exit_hotkey = "None".to_string();
                                } else {
                                    exit_hotkey = format!("Key{}", key);
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
                                    let Some(key) = ui
                                        .io()
                                        .keys_down.iter()
                                        .position(|&down| down)
                                {
                                    hotkey_key = format!("Key{}", key);
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
                            mouse_method = 0;
                        }
                        ui.same_line();
                        if ui.radio_button("ghubmouse", &mut method, 1) {
                            mouse_method = 1;
                        }
                        if method != mouse_method {
                            mouse_method = method;
                            mouse_input
                                .lock()
                                .unwrap()
                                .set_current(if mouse_method == 0 { "GFCK" } else { "GhubMouse" });
                            println!(
                                "Switched mouse input method to: {}",
                                mouse_input.lock().unwrap().get_current_name()
                            );
                        }
                        // Test buttons for mouse input
                        if ui.button("Test Click") {
                            mouse_input.lock().unwrap().click(1);
                        }
                        if ui.button("Move Right") {
                            mouse_input.lock().unwrap().move_relative(100, 0);
                        }
                    }

                    // --- Settings Tab ---
                    if let Some(_tab_item_token) = ui.tab_item("Settings") {
                        if ui.button("Auto-import from GameSettings.ini") {
                            setup.get_mouse_sensitivity_settings();
                            fov = setup.get_fov() as i32;
                            sens = setup.get_sensitivity() as i32;
                            sens_1x = setup.get_sensitivity_modifier_1() as i32;
                            sens_25x = setup.get_sensitivity_modifier_25() as i32;
                            dpi = setup.get_dpi();
                        }
                        ui.separator();
                        if ui.input_int("DPI", &mut dpi).build() {
                            setup.set_dpi(dpi);
                            setup.create_config_file();
                        }
                        if ui.slider_config("FOV", 60, 90).build(&mut fov) {
                            setup.set_fov(fov);
                        }
                        if ui.slider_config("Sensitivity", 1, 100).build(&mut sens) {
                            setup.set_sensitivity(sens);
                        }
                        if ui.slider_config("1x Sensitivity", 1, 100).build(&mut sens_1x) {
                            setup.set_sensitivity_modifier_1(sens_1x);
                        }
                        if ui.slider_config("2.5x Sensitivity", 1, 100).build(&mut sens_25x) {
                            setup.set_sensitivity_modifier_25(sens_25x);
                        }
                    }

                    // End Tab Bar
                    // (No explicit end_tab_bar() needed; handled by TabBarToken drop)
                }
            });
    });

    // NOTE:
    // You need a window to render on, so you can't get rid of it.
    // What you can do is make it the exact size of your imgui window so it's invisible.
    // This is something you have to do in the backend swapchain management code.
    // I believe that the "docking" branch of ImGui has example of it. Search for "multi viewport".
    //
    // In this main.rs, the window is always sized and positioned to match the ImGui window,
    // and with the right flags (NO_TITLE_BAR, NO_RESIZE, etc.)
    // it will appear borderless and "invisible" except for the ImGui content.
    // For true borderless/frameless, ensure your backend (winit, sdl2, etc.) also sets the native window size
    // to match the ImGui window and disables window decorations if possible.
}
