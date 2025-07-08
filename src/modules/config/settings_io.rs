use super::weapon_data::{ DEFAULT_WEAPONS };
use super::settings::Settings;
use super::setup_class::Setup;
use crate::modules::core::logger::{ log_debug };

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
            let mut setup = Setup::new(false);
            setup.get_mouse_sensitivity_settings();

            let initial_settings = [
                ("GAME", "fov", setup.get_fov().to_string()),
                ("GAME", "sens", setup.get_sensitivity().to_string()),
                ("GAME", "sens_1x", setup.get_sensitivity_modifier_1().to_string()),
                ("GAME", "sens_25x", setup.get_sensitivity_modifier_25().to_string()),
                ("GAME", "dpi", "800".to_string()),
                ("MOUSE", "method", "GFCK".to_string()),
                ("RCS_HOTKEY", "exit", "END".to_string()),
            ];

            for (section, key, value) in initial_settings {
                settings.update(section, key, value);
            }

            for (wep_name, rpm, class) in DEFAULT_WEAPONS {
                settings.update(wep_name, "X", 0.0);
                settings.update(wep_name, "Y", 1.0);
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
        let mut map = std::collections::BTreeMap::new();
        for section in self.get_all_wep() {
            if let Some(class) = self.settings.get(&section, "class") {
                map.entry(class).or_insert_with(Vec::new).push(section);
            }
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
