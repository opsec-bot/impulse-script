#![windows_subsystem = "windows"] // Comment this line to see console output
mod modules;

use imgui::*;
use modules::input::MouseInput;
use std::collections::HashMap;
use std::time::Duration;
use modules::ui::support;
use modules::config::{ Setup, SettingsIO, WEAPON_CLASSES, stompn_recoil };
use modules::core::{
    Control,
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
use modules::core::TraceRecorder;
use std::sync::{ Arc, mpsc::{ Sender, Receiver, channel } };

#[cfg(windows)]
use winapi::um::timeapi::{ timeBeginPeriod, timeEndPeriod };
#[cfg(windows)]
use winapi::um::winuser::{ GetAsyncKeyState, VK_MBUTTON, VK_XBUTTON1, VK_XBUTTON2 };

#[cfg(windows)]
struct TimePeriodGuard;

#[cfg(windows)]
impl TimePeriodGuard {
    fn new() -> Self {
        unsafe {
            timeBeginPeriod(1);
        }
        Self
    }
}

#[cfg(windows)]
impl Drop for TimePeriodGuard {
    fn drop(&mut self) {
        unsafe {
            timeEndPeriod(1);
        }
    }
}

/// Which recoil "script" is active. All three feed the same control thread; they
/// differ only in where the compensation numbers come from:
/// - `Original`: the weapon's own user-tuned X/Y/Xmod sliders (constant pull).
/// - `Stompn`:   the STOMPN-seeded X/Y table (constant pull), without touching
///               the user's saved Original values.
/// - `Advanced`: the captured per-shot recoil pattern (smoothed playback); falls
///               back to Original constant pull when no pattern is saved.
#[derive(Copy, Clone, PartialEq)]
enum RcsMode {
    Original,
    Stompn,
    Advanced,
}

impl RcsMode {
    fn from_i32(v: i32) -> Self {
        match v {
            1 => RcsMode::Stompn,
            2 => RcsMode::Advanced,
            _ => RcsMode::Original,
        }
    }
    fn as_i32(self) -> i32 {
        match self {
            RcsMode::Original => 0,
            RcsMode::Stompn => 1,
            RcsMode::Advanced => 2,
        }
    }
    fn from_str(s: &str) -> Self {
        match s {
            "Stompn" => RcsMode::Stompn,
            "Advanced" => RcsMode::Advanced,
            _ => RcsMode::Original,
        }
    }
    fn as_str(self) -> &'static str {
        match self {
            RcsMode::Original => "Original",
            RcsMode::Stompn => "Stompn",
            RcsMode::Advanced => "Advanced",
        }
    }
}

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

/// Convert a stored capture pattern (per-shot recoil deltas in capture-time
/// mouse counts) into a cumulative compensation curve in current output counts.
/// Compensation moves opposite the recoil climb, rescaled for any DPI/sens
/// difference between capture and now, plus the per-axis refinement scales.
fn build_compensation_curve(
    points: &[(i32, i32)],
    capture_dpi: i32,
    capture_sens: i32,
    current_dpi: i32,
    current_sens: i32,
    scale_x: f32,
    scale_y: f32
) -> Vec<(f32, f32)> {
    let dpi_ratio = if capture_dpi > 0 {
        (current_dpi as f32) / (capture_dpi as f32)
    } else {
        1.0
    };
    // Mouse counts for a fixed in-game angle scale inversely with sensitivity.
    let sens_ratio = if capture_sens > 0 && current_sens > 0 {
        (capture_sens as f32) / (current_sens as f32)
    } else {
        1.0
    };
    let kx = dpi_ratio * sens_ratio * scale_x;
    let ky = dpi_ratio * sens_ratio * scale_y;

    let mut comp = Vec::with_capacity(points.len() + 1);
    comp.push((0.0, 0.0));
    let mut cx = 0.0f32;
    let mut cy = 0.0f32;
    for (dx, dy) in points {
        cx += -(*dx as f32) * kx;
        cy += -(*dy as f32) * ky;
        comp.push((cx, cy));
    }
    comp
}

/// Push the active recoil script for a weapon into the control thread. The
/// `mode` selects where the numbers come from; see `RcsMode`.
fn apply_weapon_recoil(
    control: &mut Control,
    settings_io: &SettingsIO,
    weapon: &str,
    acog: bool,
    rpm: f32,
    mode: RcsMode
) {
    let timing = if rpm > 0.0 { 4234.44 / rpm + 2.58 } else { 9.64 };

    match mode {
        RcsMode::Advanced => {
            if let Some(points) = settings_io.get_pattern(weapon, acog) {
                let (cap_dpi, cap_sens) = settings_io.get_pattern_meta(weapon, acog);
                let (scale_x, scale_y) = settings_io.get_pattern_scale(weapon, acog);
                let cur_dpi = settings_io.get_dpi();
                let cur_sens = settings_io.settings
                    .get("GAME", "sens")
                    .and_then(|v| v.parse::<i32>().ok())
                    .unwrap_or(0);
                let comp = build_compensation_curve(
                    &points,
                    cap_dpi,
                    cap_sens,
                    cur_dpi,
                    cur_sens,
                    scale_x,
                    scale_y
                );
                let shot_ms = if rpm > 0.0 { 60000.0 / rpm } else { 100.0 };
                control.set_pattern(comp, shot_ms);
            } else {
                // No captured pattern for this slot: fall back to the weapon's
                // own constant-pull values so RCS still does something.
                let (x, y, xmod) = settings_io.get_weapon_values(weapon, acog);
                control.update(x as i32, y as i32, timing, xmod);
            }
        }
        RcsMode::Stompn => {
            // STOMPN-seeded X/Y, applied live without overwriting the user's
            // saved Original values. Xmod still comes from the weapon slot.
            let (_, _, xmod) = settings_io.get_weapon_values(weapon, acog);
            let (x, y) = stompn_recoil(weapon).unwrap_or_else(|| {
                let (sx, sy, _) = settings_io.get_weapon_values(weapon, acog);
                (sx, sy)
            });
            control.update(x as i32, y as i32, timing, xmod);
        }
        RcsMode::Original => {
            let (x, y, xmod) = settings_io.get_weapon_values(weapon, acog);
            control.update(x as i32, y as i32, timing, xmod);
        }
    }
}

fn main() {
    // Initialize logging first
    if let Err(e) = init_logger() {
        eprintln!("Failed to initialize logger: {}", e);
    }

    log_debug("Starting Impulse Scripts v1.0.6");

    if let Some(log_path) = get_log_file_path() {
        log_debug(&format!("Debug output being written to: {}", log_path.display()));
    }

    #[cfg(windows)]
    let _time_period_guard = TimePeriodGuard::new();

    // --- State Initialization ---
    let mut setup = Setup::new();
    setup.get_mouse_sensitivity_settings();

    log_debug("Completed setup initialization");

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

    let mouse_input = Arc::new(unsafe {
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
    });

    // Apply persisted mouse method preference (default GFCK if missing/unknown)
    if let Some(saved_method) = settings_io.settings.get("MOUSE", "method") {
        mouse_input.set_current(&saved_method);
    }

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
    let mut toggle_hotkey = settings_io
        .get_profile_hotkey("toggle")
        .unwrap_or_else(|| "F1".to_string());
    let mut hide_hotkey = settings_io
        .get_profile_hotkey("hide")
        .unwrap_or_else(|| "F2".to_string());
    let mut always_on_top_hotkey = settings_io
        .get_profile_hotkey("always_on_top")
        .unwrap_or_else(|| "F3".to_string());
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
    // Global calibration multiplier for all recoil compensation (STOMPN-seeded
    // per-weapon defaults are relative; this sets absolute strength once).
    let mut recoil_scale = settings_io.settings
        .get("GAME", "recoil_scale")
        .and_then(|v| v.parse::<f32>().ok())
        .unwrap_or(1.0);
    // Which recoil script is active (Original / Stompn / Advanced).
    let mut rcs_mode = settings_io.settings
        .get("GAME", "rcs_mode")
        .map(|v| RcsMode::from_str(&v))
        .unwrap_or(RcsMode::Advanced);
    let mut mark_key_name = settings_io
        .get_mark_key()
        .unwrap_or_else(|| "XBUTTON1".to_string());
    let mut mark_key_vk = key_name_to_vk_code(&mark_key_name).unwrap_or(VK_XBUTTON1 as i32);
    // --- Hotkey Command Channel ---
    let (hotkey_tx, hotkey_rx): (Sender<HotkeyCommand>, Receiver<HotkeyCommand>) = channel();

    // --- Control Handler State ---
    let mut control = Control::new();
    control.set_mouse_input(Arc::clone(&mouse_input));
    control.set_dpi(dpi);
    control.set_sensitivity(sens);
    control.set_recoil_scale(recoil_scale);
    control.run_threaded();

    // --- Trace Recorder ---
    #[cfg(windows)]
    let trace_recorder = TraceRecorder::new(mark_key_vk);
    #[cfg(not(windows))]
    let trace_recorder = TraceRecorder::new(0);

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
    let mut capturing_mark_key = false;
    let mut rebinding_weapon: Option<String> = None;

    let mut ghost_manager = ProcessGhost::new();

    // --- ImGui Main Loop ---
    let mut prev_weapon: Option<String> = None;
    let mut prev_acog = false;

    support::simple_init_with_resize(
        "Impusle Scripts v1.0.6",
        move |should_run, ui, set_window_size| {
            if ghost_manager.window_handle.is_none() {
                let _ = ghost_manager.find_and_set_window_handle("Impusle Config");
            }

            // Fixed UI frame cap — the heavy work happens in the control thread.
            std::thread::sleep(Duration::from_secs_f32(1.0 / 60.0));

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
                                let rpm = weapon_rpm.get(weapon).copied().unwrap_or(600) as f32;
                                apply_weapon_recoil(
                                    &mut control,
                                    &settings_io,
                                    weapon,
                                    acog_enabled,
                                    rpm,
                                    rcs_mode
                                );
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

            if !window_visible {
                return;
            }

            ui.window("Impusle Config")
                .size(size, Condition::Always)
                .position([0.0, 0.0], Condition::Always)
                .flags(window_flags)
                .build(|| {
                    if let Some(_tab_bar_token) = ui.tab_bar("main_tabs") {
                        // --- Recoil Tab ---
                        if let Some(_tab_item_token) = ui.tab_item("Recoil") {
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
                                        let rpm = weapon_rpm
                                            .get(weapon)
                                            .copied()
                                            .unwrap_or(600) as f32;
                                        apply_weapon_recoil(
                                            &mut control,
                                            &settings_io,
                                            weapon,
                                            acog_enabled,
                                            rpm,
                                            rcs_mode
                                        );
                                    }
                                }
                            }
                            ui.same_line();
                            if rcs_enabled {
                                ui.text_colored([1.0, 0.32, 0.32, 1.0], "ON");
                            } else {
                                ui.text_disabled("off");
                            }

                            ui.separator();

                            // Weapon selection row.
                            let weapons_by_class = settings_io.get_weapons_by_class();
                            ui.set_next_item_width(220.0);
                            if
                                let Some(_combo_token) = ui.begin_combo(
                                    "Weapon",
                                    selected_weapon.as_deref().unwrap_or("Select...")
                                )
                            {
                                for class in WEAPON_CLASSES {
                                    if let Some(weapons) = weapons_by_class.get(*class) {
                                        ui.text_disabled(format!("--- {} ---", class));
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
                            if !selected_weapon.is_none() {
                                ui.same_line();
                                ui.checkbox("ACOG", &mut acog_enabled);
                            }
                            ui.same_line();
                            if ui.button("Add Weapon") {
                                add_weapon_popup = true;
                            }

                            // --- Engine (which recoil script is active) ---
                            ui.separator();
                            ui.text("Engine");
                            let mut mode_sel = rcs_mode.as_i32();
                            let mut mode_changed = false;
                            mode_changed |= ui.radio_button("Original", &mut mode_sel, 0);
                            ui.same_line();
                            mode_changed |= ui.radio_button("STOMPN", &mut mode_sel, 1);
                            ui.same_line();
                            mode_changed |= ui.radio_button("Advanced", &mut mode_sel, 2);
                            if mode_changed {
                                rcs_mode = RcsMode::from_i32(mode_sel);
                                settings_io.settings.update("GAME", "rcs_mode", rcs_mode.as_str());
                                settings_io.settings.write();
                                if rcs_enabled {
                                    if let Some(weapon) = &selected_weapon {
                                        let rpm = weapon_rpm
                                            .get(weapon)
                                            .copied()
                                            .unwrap_or(600) as f32;
                                        apply_weapon_recoil(
                                            &mut control,
                                            &settings_io,
                                            weapon,
                                            acog_enabled,
                                            rpm,
                                            rcs_mode
                                        );
                                    }
                                }
                            }
                            match rcs_mode {
                                RcsMode::Original =>
                                    ui.text_disabled("Your own X / Y / Xmod values below."),
                                RcsMode::Stompn =>
                                    ui.text_disabled(
                                        "Built-in recoil table. Tune strength with Recoil Scale."
                                    ),
                                RcsMode::Advanced =>
                                    ui.text_disabled(
                                        "Plays a pattern you record in the Capture tab."
                                    ),
                            }

                            ui.separator();

                            // --- Per-mode tuning for the selected weapon ---
                            if let Some(weapon) = &selected_weapon {
                                // Re-sync the control thread when weapon/sight changes.
                                if prev_weapon != Some(weapon.clone()) || prev_acog != acog_enabled {
                                    prev_weapon = Some(weapon.clone());
                                    prev_acog = acog_enabled;

                                    if rcs_enabled {
                                        let rpm = weapon_rpm
                                            .get(weapon)
                                            .copied()
                                            .unwrap_or(600) as f32;
                                        apply_weapon_recoil(
                                            &mut control,
                                            &settings_io,
                                            weapon,
                                            acog_enabled,
                                            rpm,
                                            rcs_mode
                                        );
                                    }
                                }

                                let rpm = weapon_rpm.get(weapon).copied().unwrap_or(600) as f32;

                                match rcs_mode {
                                    RcsMode::Original => {
                                        let (x, y, xmod_val) = settings_io.get_weapon_values(
                                            weapon,
                                            acog_enabled
                                        );
                                        let mut x = x.round() as i32;
                                        let mut y = y.round() as i32;
                                        let mut xmod_val = xmod_val.round() as i32;

                                        let mut changed = false;
                                        changed |= ui.slider_config("X", -10, 10).build(&mut x);
                                        changed |= ui.slider_config("Y", 1, 10).build(&mut y);
                                        changed |= ui
                                            .slider_config("Xmod", -1, 2)
                                            .build(&mut xmod_val);

                                        if changed {
                                            settings_io.save_weapon_values(
                                                weapon,
                                                x as f32,
                                                y as f32,
                                                xmod_val as f32,
                                                acog_enabled
                                            );
                                            if rcs_enabled {
                                                apply_weapon_recoil(
                                                    &mut control,
                                                    &settings_io,
                                                    weapon,
                                                    acog_enabled,
                                                    rpm,
                                                    rcs_mode
                                                );
                                            }
                                        }

                                        if let Some((sx, sy)) = stompn_recoil(weapon) {
                                            if ui.button("Copy STOMPN values") {
                                                let (_, _, xm) = settings_io.get_weapon_values(
                                                    weapon,
                                                    acog_enabled
                                                );
                                                settings_io.save_weapon_values(
                                                    weapon,
                                                    sx,
                                                    sy,
                                                    xm,
                                                    acog_enabled
                                                );
                                                if rcs_enabled {
                                                    apply_weapon_recoil(
                                                        &mut control,
                                                        &settings_io,
                                                        weapon,
                                                        acog_enabled,
                                                        rpm,
                                                        rcs_mode
                                                    );
                                                }
                                            }
                                            if ui.is_item_hovered() {
                                                ui.tooltip_text(
                                                    "Seed X/Y from this gun's built-in values, then fine-tune."
                                                );
                                            }
                                        }
                                    }
                                    RcsMode::Stompn => {
                                        if let Some((sx, sy)) = stompn_recoil(weapon) {
                                            ui.text(
                                                format!("Built-in pull:   X {:.0}    Y {:.0}", sx, sy)
                                            );
                                            ui.text_disabled(
                                                "Switch to Original to edit these per-gun."
                                            );
                                        } else {
                                            ui.text_disabled(
                                                "No built-in entry for this gun - using its X/Y."
                                            );
                                        }
                                    }
                                    RcsMode::Advanced => {
                                        let has_pattern = settings_io.has_pattern(
                                            weapon,
                                            acog_enabled
                                        );
                                        if has_pattern {
                                            let saved = settings_io
                                                .get_pattern(weapon, acog_enabled)
                                                .map(|p| p.len())
                                                .unwrap_or(0);
                                            ui.text_colored(
                                                [0.4, 1.0, 0.4, 1.0],
                                                format!(
                                                    "Pattern: {} shots ({})",
                                                    saved + 1,
                                                    if acog_enabled { "ACOG" } else { "non-ACOG" }
                                                )
                                            );
                                            let (mut scale_x, mut scale_y) =
                                                settings_io.get_pattern_scale(weapon, acog_enabled);
                                            let mut scale_changed = false;
                                            scale_changed |= ui
                                                .slider_config("Scale X", 0.1_f32, 2.0_f32)
                                                .display_format("%.2f")
                                                .build(&mut scale_x);
                                            scale_changed |= ui
                                                .slider_config("Scale Y", 0.1_f32, 2.0_f32)
                                                .display_format("%.2f")
                                                .build(&mut scale_y);
                                            if scale_changed {
                                                settings_io.save_pattern_scale(
                                                    weapon,
                                                    acog_enabled,
                                                    scale_x,
                                                    scale_y
                                                );
                                                if rcs_enabled {
                                                    apply_weapon_recoil(
                                                        &mut control,
                                                        &settings_io,
                                                        weapon,
                                                        acog_enabled,
                                                        rpm,
                                                        rcs_mode
                                                    );
                                                }
                                            }
                                        } else {
                                            ui.text_colored(
                                                [1.0, 0.6, 0.2, 1.0],
                                                "No pattern yet - record one in the Capture tab."
                                            );
                                        }
                                    }
                                }
                            } else {
                                ui.text_disabled("Select a weapon to tune.");
                            }

                            // --- Strength (applies to every mode) ---
                            ui.separator();
                            ui.text("Strength");
                            if
                                ui
                                    .slider_config("Recoil Scale", 0.1_f32, 3.0_f32)
                                    .display_format("%.2f")
                                    .build(&mut recoil_scale)
                            {
                                control.set_recoil_scale(recoil_scale);
                                settings_io.settings.update("GAME", "recoil_scale", recoil_scale);
                                settings_io.settings.write();
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

                        // --- Capture Tab ---
                        if let Some(_tab_item_token) = ui.tab_item("Capture") {
                            ui.text_disabled(
                                "Tune your own recoil pattern by marking shots in-game and saving the resulting trace here."
                            );
                            ui.separator();

                            ui.text("Mark Key:");
                            ui.same_line();
                            if ui.button(&format!("Current: {}", mark_key_name)) {
                                capturing_mark_key = true;
                            }

                            if capturing_mark_key {
                                ui.same_line();
                                ui.text("Press a key or mouse button (ESC to cancel)...");

                                let mut updated = false;

                                if let Some((imgui_key, _)) = ui
                                    .io()
                                    .keys_down.iter()
                                    .enumerate()
                                    .find(|&(_, &down)| down)
                                {
                                    if imgui_key == (imgui::Key::Escape as usize) {
                                        capturing_mark_key = false;
                                    } else {
                                        mark_key_name = modules::ui::keybinds
                                            ::imgui_key_to_name(imgui_key as u32)
                                            .to_string();
                                        updated = true;
                                    }
                                }

                                #[cfg(windows)]
                                if !updated {
                                    let mouse_candidates = [
                                        (VK_XBUTTON1 as i32, "XBUTTON1"),
                                        (VK_XBUTTON2 as i32, "XBUTTON2"),
                                        (VK_MBUTTON as i32, "MBUTTON"),
                                    ];

                                    for (vk_code, name) in mouse_candidates {
                                        if unsafe { GetAsyncKeyState(vk_code) < 0 } {
                                            mark_key_name = name.to_string();
                                            updated = true;
                                            break;
                                        }
                                    }
                                }

                                if updated {
                                    if let Some(vk_code) = key_name_to_vk_code(&mark_key_name) {
                                        mark_key_vk = vk_code;
                                        trace_recorder.set_mark_key(mark_key_vk);
                                        settings_io.save_mark_key(&mark_key_name);
                                    }
                                    capturing_mark_key = false;
                                }
                            }

                            if selected_weapon.is_none() {
                                ui.text("Select a weapon in the Recoil tab first");
                            } else {
                                let weapon = selected_weapon.as_ref().unwrap();

                                if ui.button("Start Trace") {
                                    trace_recorder.start_trace();
                                }
                                ui.same_line();
                                if ui.button("Finish Sample") {
                                    trace_recorder.finish_sample();
                                }
                                ui.same_line();
                                if ui.button("Clear") {
                                    trace_recorder.clear();
                                }

                                ui.separator();
                                let snapshot = trace_recorder.snapshot();
                                ui.text(format!("Current shots: {}", snapshot.current_marks));
                                ui.same_line();
                                ui.text(format!("Samples: {}", snapshot.samples.len()));

                                if !snapshot.current.is_empty() {
                                    ui.text("Live trace (per-shot delta):");
                                    for (i, (x, y)) in snapshot.current.iter().enumerate() {
                                        ui.text(format!("  shot {}: {},{}", i + 2, x, y));
                                    }
                                }

                                ui.separator();

                                // Pattern playback status for this (weapon, sight) slot.
                                let has_pattern = settings_io.has_pattern(weapon, acog_enabled);
                                if has_pattern {
                                    let saved = settings_io
                                        .get_pattern(weapon, acog_enabled)
                                        .map(|p| p.len())
                                        .unwrap_or(0);
                                    ui.text_colored(
                                        [0.2, 1.0, 0.2, 1.0],
                                        format!(
                                            "Saved pattern: {} shots ({})",
                                            saved + 1,
                                            if acog_enabled { "ACOG" } else { "non-ACOG" }
                                        )
                                    );
                                } else {
                                    ui.text_colored(
                                        [1.0, 0.6, 0.2, 1.0],
                                        format!(
                                            "No saved pattern for this {} slot",
                                            if acog_enabled { "ACOG" } else { "non-ACOG" }
                                        )
                                    );
                                }

                                if ui.button("Save Pattern") {
                                    // Average completed samples by shot index.
                                    if !snapshot.samples.is_empty() {
                                        let max_len = snapshot.samples
                                            .iter()
                                            .map(|s| s.len())
                                            .max()
                                            .unwrap_or(0);
                                        let mut averaged: Vec<(i32, i32)> = Vec::new();
                                        for i in 0..max_len {
                                            let mut sx = 0i64;
                                            let mut sy = 0i64;
                                            let mut count = 0i64;
                                            for s in &snapshot.samples {
                                                if i < s.len() {
                                                    sx += s[i].0 as i64;
                                                    sy += s[i].1 as i64;
                                                    count += 1;
                                                }
                                            }
                                            if count > 0 {
                                                averaged.push((
                                                    (sx / count) as i32,
                                                    (sy / count) as i32,
                                                ));
                                            }
                                        }

                                        settings_io.save_pattern(
                                            weapon,
                                            acog_enabled,
                                            &averaged,
                                            dpi,
                                            sens
                                        );

                                        // If RCS is live on this weapon, load the new
                                        // pattern into the control thread immediately.
                                        if rcs_enabled {
                                            let rpm = weapon_rpm
                                                .get(weapon)
                                                .copied()
                                                .unwrap_or(600) as f32;
                                            apply_weapon_recoil(
                                                &mut control,
                                                &settings_io,
                                                weapon,
                                                acog_enabled,
                                                rpm,
                                                rcs_mode
                                            );
                                        }
                                    }
                                }
                                ui.same_line();
                                if has_pattern && ui.button("Delete Pattern") {
                                    settings_io.save_pattern(weapon, acog_enabled, &[], dpi, sens);
                                    if rcs_enabled {
                                        let rpm = weapon_rpm
                                            .get(weapon)
                                            .copied()
                                            .unwrap_or(600) as f32;
                                        apply_weapon_recoil(
                                            &mut control,
                                            &settings_io,
                                            weapon,
                                            acog_enabled,
                                            rpm,
                                            rcs_mode
                                        );
                                    }
                                }

                                if has_pattern {
                                    ui.text_disabled(
                                        "Tune Scale X / Y for this pattern in the Recoil tab."
                                    );
                                }

                                ui.separator();
                                ui.text("Last sample preview:");
                                let last = snapshot.samples.last();
                                if let Some(s) = last {
                                    if s.is_empty() {
                                        ui.text("Marked shots were saved, but no movement points were captured yet.");
                                    } else {
                                        for (i, (x,y)) in s.iter().enumerate() {
                                            ui.text(format!("{}: {},{}", i+1, x, y));
                                        }

                                        // Simple visualization: cumulative X/Y trajectories plotted
                                        let mut cum_x: i32 = 0;
                                        let mut cum_y: i32 = 0;
                                        let mut x_vals: Vec<f32> = Vec::with_capacity(s.len());
                                        let mut y_vals: Vec<f32> = Vec::with_capacity(s.len());
                                        for (dx, dy) in s.iter() {
                                            cum_x += *dx;
                                            cum_y += *dy;
                                            x_vals.push(cum_x as f32);
                                            y_vals.push(cum_y as f32);
                                        }

                                        if !x_vals.is_empty() {
                                            ui.spacing();
                                            ui.text("Trajectory plots:");
                                            let _ = ui.plot_lines("X Trajectory##xplot", &x_vals);
                                            let _ = ui.plot_lines("Y Trajectory##yplot", &y_vals);
                                        }
                                    }
                                } else {
                                    ui.text("No samples saved yet");
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
                                        let rpm = weapon_rpm
                                            .get(weapon)
                                            .copied()
                                            .unwrap_or(600) as f32;
                                        apply_weapon_recoil(
                                            &mut control,
                                            &settings_io,
                                            weapon,
                                            acog_enabled,
                                            rpm,
                                            rcs_mode
                                        );
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

                            ui.separator();
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
                                            let rpm = weapon_rpm
                                                .get(weapon)
                                                .copied()
                                                .unwrap_or(600) as f32;
                                            apply_weapon_recoil(
                                                &mut control,
                                                &settings_io,
                                                weapon,
                                                acog_enabled,
                                                rpm,
                                                rcs_mode
                                            );
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
                                mouse_input.set_current(if method == 0 { "GFCK" } else { "GhubMouse" });
                                settings_io.settings.update("MOUSE", "method", if method == 0 {
                                    "GFCK"
                                } else {
                                    "GhubMouse"
                                });
                                settings_io.settings.write();
                                log_debug(
                                    &format!(
                                        "Switched mouse input method to: {}",
                                        mouse_input.get_current_name()
                                    )
                                );
                                mouse_method = method;
                            }
                        }

                        // --- About Tab ---
                        if let Some(_tab_item_token) = ui.tab_item("About") {
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
