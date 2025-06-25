use crate::modules::settings::Settings;
use crate::modules::setup_class::Setup;

pub struct SettingsIO {
    pub settings: Settings,
}

impl SettingsIO {
    pub fn new() -> Self {
        let config_path = "./config.ini";
        let mut settings = Settings::new(config_path);
        if !std::path::Path::new(config_path).exists() {
            // Import game settings
            let mut setup = Setup::new(false);
            setup.get_mouse_sensitivity_settings();
            settings.update("GAME", "fov", setup.get_fov());
            settings.update("GAME", "sens", setup.get_sensitivity());
            settings.update("GAME", "sens_1x", setup.get_sensitivity_modifier_1());
            settings.update("GAME", "sens_25x", setup.get_sensitivity_modifier_25());
            settings.update("GAME", "dpi", 800);
            settings.update("MOUSE", "method", "GFCK");
            settings.update("RCS_HOTKEY", "exit", "END");

            // Default weapons (Wep, RPM, Class)
            let default_weapons: Vec<(&'static str, i32, &'static str)> = vec![
                // ARs
                ("416-C", 740, "AR"),
                ("552 COMMANDO", 690, "AR"),
                ("556XI", 690, "AR"),
                ("AK-12", 850, "AR"),
                ("AK-74M", 650, "AR"),
                ("AR33", 749, "AR"),
                ("ARX200", 700, "AR"),
                ("AUG A2", 720, "AR"),
                ("C7E", 800, "AR"),
                ("C8-SFW", 837, "AR"),
                ("F2", 980, "AR"),
                ("G36C", 780, "AR"),
                ("L85A2", 670, "AR"),
                ("M4", 750, "AR"),
                ("M762", 730, "AR"),
                ("R4-C", 860, "AR"),
                ("TYPE-89", 850, "AR"),
                // LMGs
                ("6P41", 680, "LMG"),
                ("ALDA 5.56", 900, "LMG"),
                ("DP27", 550, "LMG"),
                ("G8A1", 850, "LMG"),
                ("LMG-E", 650, "LMG"),
                ("M249 SAW", 650, "LMG"),
                ("M249", 650, "LMG"),
                ("T-95 LSW", 650, "LMG"),
                // SMGs
                ("9mm C1", 1100, "SMG"),
                ("9x19VSN", 750, "SMG"),
                ("AUG A3", 800, "SMG"),
                ("FMG-9", 800, "SMG"),
                ("K1A", 900, "SMG"),
                ("M12", 650, "SMG"),
                ("MP5", 800, "SMG"),
                ("MP5K", 800, "SMG"),
                ("MP7", 900, "SMG"),
                ("P90", 970, "SMG"),
                ("PDW9", 600, "SMG"),
                ("SCORPION EVO 3 A1", 1080, "SMG"),
                ("T-5 SMG", 900, "SMG"),
                ("UMP45", 800, "SMG"),
                ("UZK50GI", 700, "SMG"),
                ("VECTOR .45 ACP", 1200, "SMG"),
                // MPs
                ("SMG-11", 1270, "MP")
            ];
            for (wep_name, rpm, class) in default_weapons {
                settings.update(wep_name, "X", 0.0);
                settings.update(wep_name, "Y", 1.0);
                settings.update(wep_name, "RPM", rpm);
                settings.update(wep_name, "xmod", 0.0);
                settings.update(wep_name, "class", class);
            }
            settings.write();
        } else {
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
                // Lowercase for case-insensitive comparison
                let s = section.to_ascii_lowercase();
                // Exclude known config sections
                if s == "game" || s == "mouse" || s == "rcs_hotkey" {
                    return false;
                }
                // Only include if it has weapon keys (rpm, x, y, xmod)
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
}
