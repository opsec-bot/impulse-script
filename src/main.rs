#![windows_subsystem = "windows"] // Comment this line to see console output
mod modules;

use imgui::*;
use modules::input::{ MouseInput, MouseCommand };
use std::collections::HashMap;
use std::time::{ Duration, Instant };
use modules::ui::support;
use modules::config::{ Setup, SettingsIO, WEAPON_CLASSES };
use modules::core::{
    Control,
    XmodState,
    HotkeyHandler,
    HotkeyCommand,
    key_name_to_vk_code,
    ProcessGhost,
    init_logger,
    log_debug,
    log_warning,
    log_fatal,
    get_log_file_path,
};
use std::sync::{ Arc, Mutex, mpsc::{ Sender, Receiver, channel } };

fn calculate_recoil_adjustment(old_sensitivity: i32, new_sensitivity: i32, movement: f32) -> f32 {
    if new_sensitivity == 0 {
        return movement;
    }

    let constant_factor = (old_sensitivity as f32) * movement;

    constant_factor / (new_sensitivity as f32)
}

/// Updates all weapon recoil values based on sensitivity change
fn update_all_weapon_recoil_for_sensitivity(
    settings_io: &mut SettingsIO,
    old_sensitivity: i32,
    new_sensitivity: i32,
    all_weapons: &[String]
) {
    if old_sensitivity == new_sensitivity || old_sensitivity == 0 {
        return;
    }

    log_debug(
        &format!(
            "Updating all weapon recoil for sensitivity change: {} -> {}",
            old_sensitivity,
            new_sensitivity
        )
    );

    for weapon in all_weapons {
        let (x, y, xmod) = settings_io.get_weapon_values(weapon, false);
        let new_x = calculate_recoil_adjustment(old_sensitivity, new_sensitivity, x);
        let new_y = calculate_recoil_adjustment(old_sensitivity, new_sensitivity, y);

        settings_io.save_weapon_values(weapon, new_x, new_y, xmod, false);

        let (x_acog, y_acog, xmod_acog) = settings_io.get_weapon_values(weapon, true);
        if x_acog != 0.0 || y_acog != 1.0 {
            let new_x_acog = calculate_recoil_adjustment(old_sensitivity, new_sensitivity, x_acog);
            let new_y_acog = calculate_recoil_adjustment(old_sensitivity, new_sensitivity, y_acog);

            settings_io.save_weapon_values(weapon, new_x_acog, new_y_acog, xmod_acog, true);
        }
    }

    log_debug(&format!("Successfully updated recoil values for {} weapons", all_weapons.len()));
}

fn main() {
    // Initialize logging first
    if let Err(e) = init_logger() {
        eprintln!("Failed to initialize logger: {}", e);
    }

    log_debug("Starting Impulse Scripts v1.0.4");

    if let Some(log_path) = get_log_file_path() {
        log_debug(&format!("Debug output being written to: {}", log_path.display()));
    }

    // --- State Initialization ---
    let mut setup = Setup::new();
    setup.get_mouse_sensitivity_settings();

    log_debug("Completed setup initialization");

    // --- Dynamic Frame Cap State ---
    let mut last_activity = Instant::now();

    let mut settings_io = SettingsIO::new();

    let gfck_path = std::path::PathBuf::from("lib/GFCK.dll");
    let ghub_path = std::path::PathBuf::from("lib/ghub_mouse.dll");

    // Validate mouse input DLLs
    if !gfck_path.exists() {
        log_warning(&format!("GFCK.dll not found at {}", gfck_path.display()));
    }
    if !ghub_path.exists() {
        log_warning(&format!("ghub_mouse.dll not found at {}", ghub_path.display()));
    }

    let mouse_input = Arc::new(
        Mutex::new(unsafe {
            match MouseInput::new(gfck_path, ghub_path) {
                Ok(input) => {
                    log_debug("Mouse input system initialized successfully");
                    input
                }
                Err(e) => {
                    log_fatal(&format!("Failed to load mouse input DLLs: {}", e));
                    panic!("Failed to load mouse input DLLs: {}", e);
                }
            }
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

    let mut selected_weapon: Option<String> = None;
    let mut acog_enabled = false;

    let mut add_weapon_popup = false;
    let mut new_weapon_name = String::new();
    let mut new_weapon_rpm = 600;
    let mut new_weapon_class = String::new();
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
    let mut previous_sensitivity = sens;
    let mut sens_1x = setup.get_sensitivity_modifier_1() as i32;
    let mut sens_25x = setup.get_sensitivity_modifier_25() as i32;
    let (tx, rx): (Sender<MouseCommand>, Receiver<MouseCommand>) = channel();

    // --- Hotkey Command Channel ---
    let (hotkey_tx, hotkey_rx): (Sender<HotkeyCommand>, Receiver<HotkeyCommand>) = channel();

    // --- Control Handler State ---
    let mut control = Control::new();
    control.set_sender(tx);
    control.set_dpi(dpi);
    control.set_sensitivity(sens);
    control.run_threaded();

    // --- Hotkey Handler State ---
    let mut hotkey_handler = HotkeyHandler::new();
    hotkey_handler.set_sender(hotkey_tx);

    if
        let Some(exit_key) = settings_io
            .get_profile_hotkey("exit")
            .and_then(|k| key_name_to_vk_code(&k))
    {
        hotkey_handler.set_exit_key(exit_key);
    }
    if
        let Some(toggle_key) = settings_io
            .get_profile_hotkey("toggle")
            .and_then(|k| key_name_to_vk_code(&k))
    {
        hotkey_handler.set_toggle_key(toggle_key);
    }
    if
        let Some(hide_key) = settings_io
            .get_profile_hotkey("hide")
            .and_then(|k| key_name_to_vk_code(&k))
    {
        hotkey_handler.set_hide_key(hide_key);
    }
    if
        let Some(always_on_top_key) = settings_io
            .get_profile_hotkey("always_on_top")
            .and_then(|k| key_name_to_vk_code(&k))
    {
        hotkey_handler.set_always_on_top_key(always_on_top_key);
    }

    for (weapon, key_name) in settings_io.get_all_weapon_hotkeys() {
        if let Some(key_code) = key_name_to_vk_code(&key_name) {
            hotkey_handler.bind_weapon(key_code, weapon);
        }
    }

    // --- Application State ---
    let mut rcs_enabled = false;
    let mut window_visible = true;
    let mut ghost_mode_active = false;
    let mut always_on_top_active = false;

    let mut capturing_exit = false;
    let mut capturing_toggle = false;
    let mut capturing_hide = false;
    let mut capturing_always_on_top = false;
    let mut capturing_hotkey = false;
    let mut capturing_rebind = false;
    let mut rebinding_weapon: Option<String> = None;

    let mut ghost_manager = ProcessGhost::new();

    // --- ImGui Main Loop ---
    let mut xmod_state = XmodState { x_flip: 1, x_once_done: false };
    let mut prev_weapon: Option<String> = None;
    let mut prev_acog = false;

    support::simple_init_with_resize(
        "Impusle Scripts v1.0.4",
        move |should_run, ui, set_window_size| {
            if ghost_manager.window_handle.is_none() {
                let _ = ghost_manager.find_and_set_window_handle("Impusle Config");
            }

            // Dynamic frame cap
            let idle = last_activity.elapsed() > Duration::from_secs(15);

            let current_frame_cap = if rcs_enabled && !idle { 100 } else { 30 };
            let frame_time = Duration::from_secs_f32(1.0 / (current_frame_cap as f32));
            std::thread::sleep(frame_time);

            let window_flags =
                WindowFlags::NO_RESIZE |
                WindowFlags::NO_BRING_TO_FRONT_ON_FOCUS |
                WindowFlags::NO_MOVE |
                WindowFlags::NO_TITLE_BAR;

            let size = [600.0, 420.0];
            set_window_size(size);

            hotkey_handler.check_hotkeys();

            while let Ok(cmd) = hotkey_rx.try_recv() {
                match cmd {
                    HotkeyCommand::Exit => {
                        log_debug("Exit hotkey pressed");
                        *should_run = false;
                    }
                    HotkeyCommand::ToggleRcs => {
                        rcs_enabled = !rcs_enabled;
                        log_debug(
                            &format!("RCS toggled: {}", if rcs_enabled { "ON" } else { "OFF" })
                        );
                        if !rcs_enabled {
                            control.reset();
                        } else {
                            if let Some(weapon) = &selected_weapon {
                                let (x, y, xmod_val) = settings_io.get_weapon_values(
                                    weapon,
                                    acog_enabled
                                );
                                let rpm = weapon_rpm.get(weapon).copied().unwrap_or(600) as f32;
                                let timing = (4234.44 / rpm + 2.58).round() as i32;
                                control.update(x as i32, y as i32, timing, xmod_val);
                            }
                        }
                    }
                    HotkeyCommand::HideToggle => {
                        log_debug(
                            &format!("Ghost mode toggled: {}", if !ghost_mode_active {
                                "ENABLED"
                            } else {
                                "DISABLED"
                            })
                        );
                        if ghost_mode_active {
                            let _ = ghost_manager.show_in_alt_tab();
                            let _ = ghost_manager.show_in_screen_capture();
                            window_visible = true;
                            ghost_mode_active = false;
                        } else {
                            let _ = ghost_manager.hide_from_alt_tab();
                            let _ = ghost_manager.hide_from_screen_capture();
                            ghost_mode_active = true;
                        }
                    }
                    HotkeyCommand::AlwaysOnTopToggle => {
                        always_on_top_active = !always_on_top_active;
                        log_debug(
                            &format!("Always on top toggled: {}", if always_on_top_active {
                                "ENABLED"
                            } else {
                                "DISABLED"
                            })
                        );
                        let _ = ghost_manager.set_always_on_top(always_on_top_active);
                    }
                    HotkeyCommand::SelectWeapon(weapon_name) => {
                        if rcs_enabled && all_weapons.contains(&weapon_name) {
                            log_debug(&format!("Weapon selected via hotkey: {}", weapon_name));
                            selected_weapon = Some(weapon_name.clone());
                        }
                    }
                }
            }

            while let Ok(cmd) = rx.try_recv() {
                match cmd {
                    MouseCommand::Move(mut x, y) => {
                        last_activity = Instant::now();
                        if rcs_enabled {
                            if let Some(selected) = selected_weapon.as_ref() {
                                let (_, _, xmod_val) = settings_io.get_weapon_values(
                                    selected,
                                    acog_enabled
                                );
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
                                    1 => {}
                                    _ => {
                                        x = ((x as f32) * xmod_val) as i32;
                                    }
                                }
                                mouse_input.lock().unwrap().move_relative(x, y);
                            }
                        }
                    }
                    MouseCommand::Click(b) => mouse_input.lock().unwrap().click(b),
                    MouseCommand::Down(b) => mouse_input.lock().unwrap().down(b),
                    MouseCommand::Up(b) => mouse_input.lock().unwrap().up(b),
                }
            }

            if !window_visible {
                return;
            }

            ui.window("Impusle Config")
                .size(size, Condition::Always)
                .position([0.0, 0.0], Condition::Always)
                .flags(window_flags)
                .build(|| {
                    if let Some(_tab_bar_token) = ui.tab_bar("main_tabs") {
                        // --- Recoil Control Tab ---
                        if let Some(_tab_item_token) = ui.tab_item("Recoil Control") {
                            {
                                let _color_token = if rcs_enabled {
                                    ui.push_style_color(
                                        imgui::StyleColor::Text,
                                        [1.0, 0.2, 0.2, 1.0]
                                    )
                                } else {
                                    ui.push_style_color(
                                        imgui::StyleColor::Text,
                                        [0.2, 1.0, 0.2, 1.0]
                                    )
                                };
                                if
                                    ui.button(
                                        if rcs_enabled {
                                            "Disable RCS"
                                        } else {
                                            "Enable RCS"
                                        }
                                    )
                                {
                                    rcs_enabled = !rcs_enabled;
                                    if !rcs_enabled {
                                        control.reset();
                                    } else if let Some(weapon) = &selected_weapon {
                                        let (x, y, xmod_val) = settings_io.get_weapon_values(
                                            weapon,
                                            acog_enabled
                                        );
                                        let rpm = weapon_rpm
                                            .get(weapon)
                                            .copied()
                                            .unwrap_or(600) as f32;
                                        let timing = (4234.44 / rpm + 2.58).round() as i32;
                                        control.update(x as i32, y as i32, timing, xmod_val);
                                    }
                                }
                            }
                            ui.same_line();

                            let acog_label = String::from("ACOG");
                            if !selected_weapon.is_none() {
                                ui.checkbox(&acog_label, &mut acog_enabled);
                                ui.same_line();
                            }

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
                                for class in WEAPON_CLASSES {
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
                            let button_x;
                            if selected_weapon.is_none() {
                                button_x = dropdown_x + combo_width - button_width + 110.0;
                            } else {
                                button_x = dropdown_x + combo_width - button_width + 169.0;
                            }

                            let button_y = dropdown_y + 30.0 - 33.0;

                            ui.set_cursor_pos([button_x, button_y]);
                            if ui.button("Add Weapon") {
                                add_weapon_popup = true;
                            }

                            // X/Y/Xmod Sliders for selected weapon
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

                                    if rcs_enabled {
                                        let rpm = weapon_rpm
                                            .get(weapon)
                                            .copied()
                                            .unwrap_or(600) as f32;
                                        let timing = (4234.44 / rpm + 2.58).round() as i32;
                                        control.update(x as i32, y as i32, timing, xmod_val);
                                        let _ = control.current(true);
                                    }
                                }

                                let (mut x, mut y, mut xmod_val) = settings_io.get_weapon_values(
                                    weapon,
                                    acog_enabled
                                );

                                let mut changed = false;
                                changed |= ui.slider_config("X", -2.0, 2.0).build(&mut x);
                                changed |= ui.slider_config("Y", 1.0, 10.0).build(&mut y);
                                changed |= ui.slider_config("Xmod", -1.0, 2.0).build(&mut xmod_val);

                                if changed {
                                    settings_io.save_weapon_values(
                                        weapon,
                                        x,
                                        y,
                                        xmod_val,
                                        acog_enabled
                                    );
                                    if rcs_enabled {
                                        let rpm = weapon_rpm
                                            .get(weapon)
                                            .copied()
                                            .unwrap_or(600) as f32;
                                        let timing = (4234.44 / rpm + 2.58).round() as i32;
                                        control.update(x as i32, y as i32, timing, xmod_val);
                                        let _ = control.current(true);
                                    }
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
                                    for class in WEAPON_CLASSES {
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
                            ui.text("Exit:");

                            ui.same_line();
                            if ui.button(&format!("Current: {}", exit_hotkey)) {
                                capturing_exit = true;
                            }

                            if capturing_exit {
                                ui.same_line();
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
                                    if let Some(key_code) = key_name_to_vk_code(&exit_hotkey) {
                                        hotkey_handler.set_exit_key(key_code);
                                    }
                                    capturing_exit = false;
                                }
                            }

                            let mut toggle_hotkey = settings_io
                                .get_profile_hotkey("toggle")
                                .unwrap_or_else(|| "F1".to_string());

                            ui.text("Toggle Script:");

                            ui.same_line();
                            if ui.button(&format!("Current: {}", toggle_hotkey)) {
                                capturing_toggle = true;
                            }

                            if capturing_toggle {
                                ui.same_line();
                                ui.text("Press a key (ESC to clear)...");
                                if
                                    let Some((imgui_key, _)) = ui
                                        .io()
                                        .keys_down.iter()
                                        .enumerate()
                                        .find(|&(_, &down)| down)
                                {
                                    if imgui_key == (imgui::Key::Escape as usize) {
                                        toggle_hotkey = "None".to_string();
                                    } else {
                                        toggle_hotkey = modules::ui::keybinds
                                            ::imgui_key_to_name(imgui_key as u32)
                                            .to_string();
                                    }
                                    settings_io.save_profile_hotkey("toggle", &toggle_hotkey);
                                    if let Some(key_code) = key_name_to_vk_code(&toggle_hotkey) {
                                        hotkey_handler.set_toggle_key(key_code);
                                    }
                                    capturing_toggle = false;
                                }
                            }

                            let mut hide_hotkey = settings_io
                                .get_profile_hotkey("hide")
                                .unwrap_or_else(|| "F2".to_string());

                            ui.text("Ghost Mode:");

                            ui.same_line();
                            if ui.button(&format!("Current: {}", hide_hotkey)) {
                                capturing_hide = true;
                            }

                            if capturing_hide {
                                ui.same_line();
                                ui.text("Press a key (ESC to clear)...");
                                if
                                    let Some((imgui_key, _)) = ui
                                        .io()
                                        .keys_down.iter()
                                        .enumerate()
                                        .find(|&(_, &down)| down)
                                {
                                    if imgui_key == (imgui::Key::Escape as usize) {
                                        hide_hotkey = "None".to_string();
                                    } else {
                                        hide_hotkey = modules::ui::keybinds
                                            ::imgui_key_to_name(imgui_key as u32)
                                            .to_string();
                                    }
                                    settings_io.save_profile_hotkey("hide", &hide_hotkey);
                                    if let Some(key_code) = key_name_to_vk_code(&hide_hotkey) {
                                        hotkey_handler.set_hide_key(key_code);
                                    }
                                    capturing_hide = false;
                                }
                            }

                            let mut always_on_top_hotkey = settings_io
                                .get_profile_hotkey("always_on_top")
                                .unwrap_or_else(|| "F3".to_string());

                            ui.text("Top most:");

                            ui.same_line();
                            if ui.button(&format!("Current: {}", always_on_top_hotkey)) {
                                capturing_always_on_top = true;
                            }

                            if capturing_always_on_top {
                                ui.same_line();
                                ui.text("Press a key (ESC to clear)...");
                                if
                                    let Some((imgui_key, _)) = ui
                                        .io()
                                        .keys_down.iter()
                                        .enumerate()
                                        .find(|&(_, &down)| down)
                                {
                                    if imgui_key == (imgui::Key::Escape as usize) {
                                        always_on_top_hotkey = "None".to_string();
                                    } else {
                                        always_on_top_hotkey = modules::ui::keybinds
                                            ::imgui_key_to_name(imgui_key as u32)
                                            .to_string();
                                    }
                                    settings_io.save_profile_hotkey(
                                        "always_on_top",
                                        &always_on_top_hotkey
                                    );
                                    if
                                        let Some(key_code) = key_name_to_vk_code(
                                            &always_on_top_hotkey
                                        )
                                    {
                                        hotkey_handler.set_always_on_top_key(key_code);
                                    }
                                    capturing_always_on_top = false;
                                }
                            }
                            ui.separator();

                            // --- Weapon Hotkeys ---
                            ui.text("Weapon Hotkeys:");
                            let weapon_hotkeys = settings_io.get_all_weapon_hotkeys();
                            let mut weapons_to_remove = Vec::new();
                            let mut weapons_to_rebind = Vec::new();

                            for (weapon, key) in &weapon_hotkeys {
                                ui.text(format!("{}: {}", weapon, key));
                                ui.same_line();
                                if ui.button(&format!("Rebind##{}", weapon)) {
                                    weapons_to_rebind.push(weapon.clone());
                                }
                                ui.same_line();
                                if ui.button(&format!("Remove##{}", weapon)) {
                                    weapons_to_remove.push(weapon.clone());
                                }
                            }

                            for weapon in weapons_to_rebind {
                                rebinding_weapon = Some(weapon);
                                capturing_rebind = true;
                                break;
                            }

                            if capturing_rebind {
                                if let Some(ref weapon) = rebinding_weapon {
                                    ui.text(
                                        &format!("Rebinding {}: Press a key (ESC to cancel)...", weapon)
                                    );
                                    if
                                        let Some((imgui_key, _)) = ui
                                            .io()
                                            .keys_down.iter()
                                            .enumerate()
                                            .find(|&(_, &down)| down)
                                    {
                                        if imgui_key == (imgui::Key::Escape as usize) {
                                            capturing_rebind = false;
                                            rebinding_weapon = None;
                                        } else {
                                            let new_key = modules::ui::keybinds
                                                ::imgui_key_to_name(imgui_key as u32)
                                                .to_string();
                                            settings_io.save_profile_hotkey(weapon, &new_key);
                                            if let Some(key_code) = key_name_to_vk_code(&new_key) {
                                                hotkey_handler.bind_weapon(
                                                    key_code,
                                                    weapon.clone()
                                                );
                                            }
                                            capturing_rebind = false;
                                            rebinding_weapon = None;
                                        }
                                    }
                                }
                            }

                            for weapon in weapons_to_remove {
                                if
                                    let Some((_, key)) = weapon_hotkeys
                                        .iter()
                                        .find(|(w, _)| w == &weapon)
                                {
                                    settings_io.remove_weapon_hotkey(&weapon);
                                    if let Some(key_code) = key_name_to_vk_code(key) {
                                        hotkey_handler.unbind_weapon(key_code);
                                    }
                                }
                            }

                            if ui.button("+") {
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

                                ui.input_text("Key", &mut hotkey_key).build();
                                if ui.button("Bind") {
                                    if !hotkey_weapon.is_empty() && !hotkey_key.is_empty() {
                                        settings_io.save_profile_hotkey(
                                            &hotkey_weapon,
                                            &hotkey_key
                                        );
                                        hotkey_bindings.insert(
                                            hotkey_key.clone(),
                                            hotkey_weapon.clone()
                                        );
                                        hotkey_weapon.clear();
                                        hotkey_key.clear();
                                        hotkey_add_popup = false;
                                        ui.close_current_popup();
                                    }
                                }
                                if ui.button("Cancel") {
                                    hotkey_add_popup = false;
                                    ui.close_current_popup();
                                }
                            }
                        }

                        // --- Settings Tab ---
                        if let Some(_tab_item_token) = ui.tab_item("Settings") {
                            if ui.input_int("DPI", &mut dpi).build() {
                                settings_io.set_dpi(dpi);
                                control.set_dpi(dpi);
                            }

                            if ui.slider_config("FOV", 60, 90).build(&mut fov) {
                                settings_io.settings.update("GAME", "fov", fov);
                                settings_io.settings.write();
                            }

                            if ui.slider_config("Sensitivity", 1, 100).build(&mut sens) {
                                control.set_sensitivity(sens);

                                update_all_weapon_recoil_for_sensitivity(
                                    &mut settings_io,
                                    previous_sensitivity,
                                    sens,
                                    &all_weapons
                                );

                                if rcs_enabled {
                                    if let Some(weapon) = &selected_weapon {
                                        let (x, y, xmod_val) = settings_io.get_weapon_values(
                                            weapon,
                                            acog_enabled
                                        );
                                        let rpm = weapon_rpm
                                            .get(weapon)
                                            .copied()
                                            .unwrap_or(600) as f32;
                                        let timing = (4234.44 / rpm + 2.58).round() as i32;
                                        control.update(x as i32, y as i32, timing, xmod_val);
                                    }
                                }

                                previous_sensitivity = sens;
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

                            if ui.button("Auto-import from GameSettings.ini") {
                                let old_sens = sens;

                                setup.get_mouse_sensitivity_settings();
                                fov = setup.get_fov() as i32;
                                sens = setup.get_sensitivity() as i32;
                                sens_1x = setup.get_sensitivity_modifier_1() as i32;
                                sens_25x = setup.get_sensitivity_modifier_25() as i32;

                                settings_io.settings.update("GAME", "fov", fov);
                                settings_io.settings.update("GAME", "sens", sens);
                                settings_io.settings.update("GAME", "sens_1x", sens_1x);
                                settings_io.settings.update("GAME", "sens_25x", sens_25x);
                                control.set_sensitivity(sens);

                                if old_sens != sens && old_sens != 0 {
                                    update_all_weapon_recoil_for_sensitivity(
                                        &mut settings_io,
                                        old_sens,
                                        sens,
                                        &all_weapons
                                    );

                                    if rcs_enabled {
                                        if let Some(weapon) = &selected_weapon {
                                            let (x, y, xmod_val) = settings_io.get_weapon_values(
                                                weapon,
                                                acog_enabled
                                            );
                                            let rpm = weapon_rpm
                                                .get(weapon)
                                                .copied()
                                                .unwrap_or(600) as f32;
                                            let timing = (4234.44 / rpm + 2.58).round() as i32;
                                            control.update(x as i32, y as i32, timing, xmod_val);
                                        }
                                    }
                                }

                                previous_sensitivity = sens;
                            }

                            ui.separator();
                            ui.text("Mouse Input Method:");
                            let mut method = mouse_method;
                            if
                                ui.radio_button("GFCK", &mut method, 0) ||
                                ui.radio_button("GhubMouse", &mut method, 1)
                            {
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
                                log_debug(
                                    &format!(
                                        "Switched mouse input method to: {}",
                                        mouse_input.lock().unwrap().get_current_name()
                                    )
                                );
                                mouse_method = method;
                            }
                        }

                        // --- Extras Tab ---
                        if let Some(_tab_item_token) = ui.tab_item("Extras") {
                            ui.text("Developed by credit132 on unknowncheats.me");
                            ui.separator();
                            ui.text("Program Information:");
                            let fps = ui.io().framerate;
                            ui.text(format!("FPS: {:.1}", fps));
                            ui.text("Ghost Status:");
                            ui.same_line();
                            if ghost_mode_active {
                                ui.text_colored([0.0, 1.0, 0.0, 1.0], "ACTIVE");
                            } else {
                                ui.text_colored([1.0, 0.5, 0.0, 1.0], "DISABLED");
                            }
                            ui.text("Always On Top:");
                            ui.same_line();
                            if always_on_top_active {
                                ui.text_colored([0.0, 1.0, 0.0, 1.0], "ACTIVE");
                            } else {
                                ui.text_colored([1.0, 0.5, 0.0, 1.0], "DISABLED");
                            }
                        }
                    }
                });
        }
    );

    log_debug("Application shutting down");
}
