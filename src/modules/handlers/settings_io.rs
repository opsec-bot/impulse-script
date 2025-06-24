use crate::modules::handlers::settings::Settings;
pub struct SettingsIO {
    pub settings: Settings,
}

impl SettingsIO {
    pub const HOTKEY: &'static str = "RCS_HOTKEY";
    pub const RCS: &'static str = "RCS";

    pub fn new() -> Self {
        let mut settings = Settings::new("./config.ini");
        settings.read();
        println!("{:?}", settings.sections());
        Self { settings }
    }

    pub fn get_xyt(&self, category: &str) -> Option<(i32, i32, i32)> {
        let x = self.settings.get(category, "X")?.parse().ok()?;
        let y = self.settings.get(category, "Y")?.parse().ok()?;
        let timing = self.settings.get(category, "Timing")?.parse().ok()?;
        Some((x, y, timing))
    }

    pub fn get_all_profiles(&self) -> Vec<String> {
        self.settings
            .sections()
            .into_iter()
            .filter(|s| !s.starts_with("RCS"))
            .collect()
    }

    pub fn save_profile(&mut self, category: &str, combined: &str) {
        self.settings.update(category, "combined", combined);
        if let Some((x, y, t, x_mod)) = Self::parse_combined(combined) {
            self.settings.update(category, "X", x.to_string());
            self.settings.update(category, "Y", y.to_string());
            self.settings.update(category, "Timing", t.to_string());
            self.settings.update(category, "x_mod", x_mod.to_string());
        }
        self.settings.write();
    }

    pub fn save_profile_hotkey(&mut self, category: &str, hotkey: &str) {
        self.settings.update(Self::HOTKEY, hotkey, category);
        self.settings.write();
    }

    pub fn get_profile_hotkey(&self, category: &str) -> Option<String> {
        self.settings.get(Self::HOTKEY, category)
    }

    pub fn get_profile_from_hotkey(&self, hotkey: &str) -> Option<String> {
        if self.settings.options(Self::HOTKEY).contains(&hotkey.to_string()) {
            self.settings.get(Self::HOTKEY, hotkey)
        } else {
            None
        }
    }

    pub fn get_all_RCS_binds(&self) -> Vec<(String, String)> {
        self.settings
            .options(Self::RCS)
            .into_iter()
            .filter(|o| o.starts_with("bind_"))
            .map(|opt| {
                let bind = self.get_RCS(&opt).unwrap_or_default();
                let callback = opt.trim_start_matches("bind_").to_string();
                (bind, callback)
            })
            .collect()
    }

    pub fn get_RCS(&self, option: &str) -> Option<String> {
        self.settings.get(Self::RCS, option)
    }

    pub fn set_RCS(&mut self, option: &str, value: &str) {
        self.settings.update(Self::RCS, option, value);
    }

    pub fn add_timing(&mut self, weapon: &str, class_name: &str, timing: i32) {
        let key = format!("{}_timings", class_name);
        let raw = self.get_RCS(&key).unwrap_or_else(|| "{}".into());
        let mut map: std::collections::BTreeMap<String, i32> = serde_json
            ::from_str(&raw)
            .unwrap_or_default();
        map.insert(weapon.to_string(), timing);
        let updated = serde_json::to_string(&map).unwrap();
        self.set_RCS(&key, &updated);
        self.settings.write();
    }

    pub fn category_to_map(&self, category: &str) -> std::collections::HashMap<String, String> {
        self.settings.config
            .get_map()
            .and_then(|map| map.get(category).cloned())
            .map(|hm| {
                hm.into_iter()
                    .filter_map(|(k, v)| v.map(|val| (k, val)))
                    .collect()
            })
            .unwrap_or_default()
    }

    fn parse_combined(combined: &str) -> Option<(i32, i32, i32, i32)> {
        let parts: Vec<i32> = combined
            .split(',')
            .filter_map(|x| x.trim().parse().ok())
            .collect();
        if parts.len() == 4 {
            Some((parts[0], parts[1], parts[2], parts[3]))
        } else {
            None
        }
    }
}
