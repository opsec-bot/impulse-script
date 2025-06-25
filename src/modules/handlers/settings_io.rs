use crate::modules::handlers::settings::Settings;
use crate::modules::handlers::setup_class::Setup;

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
            settings.update("GAME", "dpi", setup.get_dpi());
            settings.update("MOUSE", "method", "GFCK");
            settings.update("RCS_HOTKEY", "exit", "END");

            // Default weapons (Wep, RPM)
            let default_weapons: Vec<(&'static str, i32)> = vec![
                ("416-C", 740),
                ("552 COMMANDO", 690),
                ("556XI", 690),
                ("AK-12", 850),
                ("AK-74M", 650),
                ("AR33", 749),
                ("ARX200", 700),
                ("AUG A2", 720),
                ("C7E", 800),
                ("C8-SFW", 837),
                ("F2", 980),
                ("G36C", 780),
                ("L85A2", 670),
                ("M4", 750),
                ("M762", 730),
                ("R4-C", 860),
                ("TYPE-89", 850),
                // LMGs
                ("6P41", 680),
                ("ALDA 5.56", 900),
                ("DP27", 550),
                ("G8A1", 850),
                ("LMG-E", 650),
                ("M249 SAW", 650),
                ("M249", 650),
                ("T-95 LSW", 650),
                // SMGs
                ("9mm C1", 1100),
                ("9x19VSN", 750),
                ("AUG A3", 800),
                ("FMG-9", 800),
                ("K1A", 900),
                ("M12", 650),
                ("MP5", 800),
                ("MP5K", 800),
                ("MP7", 900),
                ("P90", 970),
                ("PDW9", 600),
                ("SCORPION EVO 3 A1", 1080),
                ("T-5 SMG", 900),
                ("UMP45", 800),
                ("UZK50GI", 700),
                ("VECTOR .45 ACP", 1200)
            ];
            for (wep_name, rpm) in default_weapons {
                settings.update(wep_name, "X", 0.0);
                settings.update(wep_name, "Y", 1.0);
                settings.update(wep_name, "RPM", rpm);
                settings.update(wep_name, "xmod", 0.0);
            }
            settings.write();
        } else {
            settings.read();
        }
        Self { settings }
    }

    pub fn get_xyt(&self, wep_name: &str) -> Option<(i32, i32, i32)> {
        let x = self.settings.get(wep_name, "X")?.parse().ok()?;
        let y = self.settings.get(wep_name, "Y")?.parse().ok()?;
        let timing = self.settings.get(wep_name, "Timing")?.parse().ok()?;
        Some((x, y, timing))
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

    pub fn save_wep(&mut self, wep_name: &str, combined: &str) {
        self.settings.update(wep_name, "combined", combined);
        if let Some((x, y, t, x_mod)) = Self::parse_combined(combined) {
            self.settings.update(wep_name, "X", x.to_string());
            self.settings.update(wep_name, "Y", y.to_string());
            self.settings.update(wep_name, "Timing", t.to_string());
            self.settings.update(wep_name, "x_mod", x_mod.to_string());
        }
        self.settings.write();
    }

    fn parse_combined(combined: &str) -> Option<(i32, i32, i32, f32)> {
        let parts: Vec<&str> = combined.split(',').collect();
        if parts.len() != 4 {
            return None;
        }
        let x = parts[0].trim().parse().ok()?;
        let y = parts[1].trim().parse().ok()?;
        let timing = parts[2].trim().parse().ok()?;
        let x_mod = parts[3].trim().parse().ok()?;
        Some((x, y, timing, x_mod))
    }

    pub fn get_profile_hotkey(&self, hotkey_name: &str) -> Option<String> {
        self.settings.get("RCS_HOTKEY", hotkey_name)
    }

    pub fn save_profile_hotkey(&mut self, hotkey_name: &str, value: &str) {
        self.settings.update("RCS_HOTKEY", hotkey_name, value);
        self.settings.write();
    }
}
