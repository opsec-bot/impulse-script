use super::weapon_data::{ DEFAULT_WEAPONS, ver2_recoil };
use super::settings::Settings;
use super::setup_class::Setup;
use crate::modules::core::logger::{ log_debug };
use std::collections::{ BTreeMap, HashMap };

pub struct SettingsIO {
    pub settings: Settings,
}

impl SettingsIO {
    pub fn new() -> Self {
        log_debug("Initializing SettingsIO");
        let config_path = "./config.ini";
        let mut settings = Settings::new(config_path);
        if !std::path::Path::new(config_path).exists() {
            log_debug("Config file not found, creating with default values");
            let mut setup = Setup::new();
            setup.get_mouse_sensitivity_settings();

            let initial_settings = [
                ("GAME", "fov", setup.get_fov().to_string()),
                ("GAME", "sens", setup.get_sensitivity().to_string()),
                ("GAME", "sens_1x", setup.get_sensitivity_modifier_1().to_string()),
                ("GAME", "sens_25x", setup.get_sensitivity_modifier_25().to_string()),
                ("GAME", "dpi", "800".to_string()),
                ("MOUSE", "method", "GFCK".to_string()),
                ("RCS_HOTKEY", "exit", "END".to_string()),
                ("RCS_HOTKEY", "toggle", "F1".to_string()),
                ("RCS_HOTKEY", "hide", "F2".to_string()),
                ("RCS_HOTKEY", "always_on_top", "F3".to_string()),
                ("RCS_HOTKEY", "mark_key", "XBUTTON1".to_string()),
            ];

            for (section, key, value) in initial_settings {
                settings.update(section, key, value);
            }

            for (wep_name, rpm, class) in DEFAULT_WEAPONS {
                let (x, y) = ver2_recoil(wep_name).unwrap_or((0.0, 1.0));
                settings.update(wep_name, "X", x);
                settings.update(wep_name, "Y", y);
                settings.update(wep_name, "RPM", *rpm);
                settings.update(wep_name, "xmod", 0.0);
                settings.update(wep_name, "class", class);
            }
            settings.write();
            log_debug("Default configuration written to file");
        } else {
            log_debug("Loading existing configuration file");
            settings.read();
        }
        Self { settings }
    }

    pub fn get_weapons_by_class(&self) -> std::collections::BTreeMap<String, Vec<String>> {
        let mut map = BTreeMap::new();
        let order_by_weapon: HashMap<&str, usize> = DEFAULT_WEAPONS
            .iter()
            .enumerate()
            .map(|(index, (weapon_name, _, _))| (*weapon_name, index))
            .collect();

        for section in self.get_all_wep() {
            if let Some(class) = self.settings.get(&section, "class") {
                map.entry(class).or_insert_with(Vec::new).push(section);
            }
        }

        for weapons in map.values_mut() {
            weapons.sort_by(|left, right| {
                let left_rank = order_by_weapon.get(left.as_str()).copied().unwrap_or(usize::MAX);
                let right_rank = order_by_weapon.get(right.as_str()).copied().unwrap_or(usize::MAX);

                left_rank.cmp(&right_rank).then_with(|| left.cmp(right))
            });
        }

        map
    }

    pub fn get_dpi(&self) -> i32 {
        self.settings
            .get("GAME", "dpi")
            .and_then(|v| v.parse().ok())
            .unwrap_or(800)
    }

    pub fn set_dpi(&mut self, dpi: i32) {
        log_debug(&format!("Updating DPI setting to: {}", dpi));
        self.settings.update("GAME", "dpi", dpi);
        self.settings.write();
    }

    pub fn get_weapon_rpm(&self, wep_name: &str) -> Option<i32> {
        self.settings.get(wep_name, "RPM").and_then(|v| v.parse().ok())
    }

    /// Overwrite each weapon's X/Y with its VER2 recoil default. Weapons absent
    /// from the table are left untouched. Returns how many weapons were updated.
    pub fn apply_ver2_defaults(&mut self) -> usize {
        let mut count = 0;
        for weapon in self.get_all_wep() {
            if let Some((x, y)) = ver2_recoil(&weapon) {
                self.settings.update(&weapon, "X", x);
                self.settings.update(&weapon, "Y", y);
                count += 1;
            }
        }
        self.settings.write();
        log_debug(&format!("Applied VER2 recoil defaults to {} weapons", count));
        count
    }

    pub fn get_weapon_values(&self, wep_name: &str, acog: bool) -> (f32, f32, f32) {
        if acog {
            let x = self.settings
                .get(wep_name, "X_acog")
                .and_then(|v| v.parse().ok())
                .unwrap_or(0.0);
            let y = self.settings
                .get(wep_name, "Y_acog")
                .and_then(|v| v.parse().ok())
                .unwrap_or(1.0);
            let xmod = self.settings
                .get(wep_name, "Xmod_acog")
                .and_then(|v| v.parse().ok())
                .unwrap_or(0.02);
            (x, y, xmod)
        } else {
            let x = self.settings
                .get(wep_name, "X")
                .and_then(|v| v.parse().ok())
                .unwrap_or(0.0);
            let y = self.settings
                .get(wep_name, "Y")
                .and_then(|v| v.parse().ok())
                .unwrap_or(1.0);
            let xmod = self.settings
                .get(wep_name, "xmod")
                .and_then(|v| v.parse().ok())
                .unwrap_or(0.02);
            (x, y, xmod)
        }
    }

    pub fn save_weapon_values(&mut self, wep_name: &str, x: f32, y: f32, xmod: f32, acog: bool) {
        let scope_suffix = if acog { "_acog" } else { "" };
        log_debug(
            &format!(
                "Saving weapon values for {}{}: X={:.2}, Y={:.2}, Xmod={:.2}",
                wep_name,
                scope_suffix,
                x,
                y,
                xmod
            )
        );

        if acog {
            self.settings.update(wep_name, "X_acog", x);
            self.settings.update(wep_name, "Y_acog", y);
            self.settings.update(wep_name, "Xmod_acog", xmod);
        } else {
            self.settings.update(wep_name, "X", x);
            self.settings.update(wep_name, "Y", y);
            self.settings.update(wep_name, "xmod", xmod);
        }
        self.settings.write();
    }

    /// Persist a captured recoil pattern (per-shot deltas in capture-time mouse
    /// counts) for a weapon + sight slot, along with the DPI/sensitivity it was
    /// captured at (needed to rescale on playback).
    pub fn save_pattern(
        &mut self,
        wep_name: &str,
        acog: bool,
        points: &[(i32, i32)],
        dpi: i32,
        sens: i32
    ) {
        let suffix = if acog { "_acog" } else { "" };
        let serialized = points
            .iter()
            .map(|(x, y)| format!("{},{}", x, y))
            .collect::<Vec<_>>()
            .join(";");
        log_debug(
            &format!(
                "Saving pattern for {}{}: {} shots @ {}dpi/{}sens",
                wep_name,
                suffix,
                points.len(),
                dpi,
                sens
            )
        );
        self.settings.update(wep_name, &format!("pattern{}", suffix), serialized);
        self.settings.update(wep_name, &format!("pattern_dpi{}", suffix), dpi);
        self.settings.update(wep_name, &format!("pattern_sens{}", suffix), sens);
        self.settings.write();
    }

    /// Parse a stored pattern back into per-shot deltas. Returns `None` if no
    /// pattern is saved for this weapon + sight slot.
    pub fn get_pattern(&self, wep_name: &str, acog: bool) -> Option<Vec<(i32, i32)>> {
        let suffix = if acog { "_acog" } else { "" };
        let raw = self.settings.get(wep_name, &format!("pattern{}", suffix))?;
        let raw = raw.trim().to_string();
        if raw.is_empty() {
            return None;
        }
        let mut points = Vec::new();
        for pair in raw.split(';') {
            let pair = pair.trim();
            if pair.is_empty() {
                continue;
            }
            let mut it = pair.split(',');
            let x = it.next().and_then(|v| v.trim().parse::<i32>().ok());
            let y = it.next().and_then(|v| v.trim().parse::<i32>().ok());
            if let (Some(x), Some(y)) = (x, y) {
                points.push((x, y));
            }
        }
        if points.is_empty() { None } else { Some(points) }
    }

    /// DPI and sensitivity a pattern was captured at. Falls back to the current
    /// configured DPI and `0` sens (meaning "unknown — assume same") when absent.
    pub fn get_pattern_meta(&self, wep_name: &str, acog: bool) -> (i32, i32) {
        let suffix = if acog { "_acog" } else { "" };
        let dpi = self.settings
            .get(wep_name, &format!("pattern_dpi{}", suffix))
            .and_then(|v| v.parse().ok())
            .unwrap_or_else(|| self.get_dpi());
        let sens = self.settings
            .get(wep_name, &format!("pattern_sens{}", suffix))
            .and_then(|v| v.parse().ok())
            .unwrap_or(0);
        (dpi, sens)
    }

    /// Per-axis playback scale refinement (defaults to 1.0/1.0).
    pub fn get_pattern_scale(&self, wep_name: &str, acog: bool) -> (f32, f32) {
        let suffix = if acog { "_acog" } else { "" };
        let sx = self.settings
            .get(wep_name, &format!("pattern_scale_x{}", suffix))
            .and_then(|v| v.parse().ok())
            .unwrap_or(1.0);
        let sy = self.settings
            .get(wep_name, &format!("pattern_scale_y{}", suffix))
            .and_then(|v| v.parse().ok())
            .unwrap_or(1.0);
        (sx, sy)
    }

    pub fn save_pattern_scale(&mut self, wep_name: &str, acog: bool, scale_x: f32, scale_y: f32) {
        let suffix = if acog { "_acog" } else { "" };
        self.settings.update(wep_name, &format!("pattern_scale_x{}", suffix), scale_x);
        self.settings.update(wep_name, &format!("pattern_scale_y{}", suffix), scale_y);
        self.settings.write();
    }

    /// Whether a usable pattern exists for this weapon + sight slot.
    pub fn has_pattern(&self, wep_name: &str, acog: bool) -> bool {
        self.get_pattern(wep_name, acog).is_some()
    }

    pub fn get_all_wep(&self) -> Vec<String> {
        self.settings
            .sections()
            .into_iter()
            .filter(|section| {
                let s = section.to_ascii_lowercase();

                if s == "game" || s == "mouse" || s == "rcs_hotkey" {
                    return false;
                }

                let has_rpm = self.settings.get(section, "rpm").is_some();
                let has_x = self.settings.get(section, "x").is_some();
                let has_y = self.settings.get(section, "y").is_some();
                let has_xmod = self.settings.get(section, "xmod").is_some();
                has_rpm && has_x && has_y && has_xmod
            })
            .collect()
    }

    pub fn get_profile_hotkey(&self, hotkey_name: &str) -> Option<String> {
        self.settings.get("RCS_HOTKEY", hotkey_name)
    }

    pub fn save_profile_hotkey(&mut self, hotkey_name: &str, value: &str) {
        self.settings.update("RCS_HOTKEY", hotkey_name, value);
        self.settings.write();
    }

    pub fn get_mark_key(&self) -> Option<String> {
        self.get_profile_hotkey("mark_key")
    }

    pub fn save_mark_key(&mut self, value: &str) {
        self.save_profile_hotkey("mark_key", value);
    }

    pub fn get_all_weapon_hotkeys(&self) -> Vec<(String, String)> {
        let mut weapon_hotkeys = Vec::new();
        for weapon in self.get_all_wep() {
            if let Some(hotkey) = self.settings.get("RCS_HOTKEY", &weapon) {
                if !hotkey.is_empty() {
                    weapon_hotkeys.push((weapon, hotkey));
                }
            }
        }
        weapon_hotkeys
    }
    pub fn remove_weapon_hotkey(&mut self, weapon_name: &str) {
        self.settings.update("RCS_HOTKEY", weapon_name, "");
        self.settings.write();
    }
}
