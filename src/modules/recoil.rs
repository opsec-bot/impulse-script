use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

pub struct ScopeSensitivityCalculator {
    pub fov: f64,
    pub sens: f64,
    pub xfactor: f64,
    pub x1modifier: f64,
    pub x15modifier: f64,
    pub x2modifier: f64,
    pub x25modifier: f64,
    pub x3modifier: f64,
    pub x4modifier: f64,
}

impl ScopeSensitivityCalculator {
    pub fn new(
        fov: f64,
        sens: f64,
        xfactor: f64,
        x1modifier: f64,
        x15modifier: f64,
        x2modifier: f64,
        x25modifier: f64,
        x3modifier: f64,
        x4modifier: f64,
    ) -> Self {
        Self {
            fov,
            sens,
            xfactor,
            x1modifier,
            x15modifier,
            x2modifier,
            x25modifier,
            x3modifier,
            x4modifier,
        }
    }

    fn calculate_ads(&self, modifier: f64, fov_multiplier: f64, ads_multiplier: f64) -> i32 {
        let fov_adjustment = ((fov_multiplier * self.fov).to_radians() / 2.0).tan()
            / (self.fov.to_radians() / 2.0).tan();
        (modifier / (ads_multiplier / fov_adjustment)
            * self.xfactor
            * ads_multiplier
            * self.sens)
            .round() as i32
    }

    pub fn calculate_ads_values(&self) -> HashMap<&'static str, i32> {
        let mut map = HashMap::new();
        map.insert("x1 ADS", self.calculate_ads(self.x1modifier, 0.9, 0.6));
        map.insert("x15 ADS", self.calculate_ads(self.x15modifier, 0.59, 0.59));
        map.insert("x2 ADS", self.calculate_ads(self.x2modifier, 0.49, 0.49));
        map.insert("x25 ADS", self.calculate_ads(self.x25modifier, 0.42, 0.42));
        map.insert("x3 ADS", self.calculate_ads(self.x3modifier, 0.35, 0.35));
        map.insert("x4 ADS", self.calculate_ads(self.x4modifier, 0.3, 0.3));
        map
    }
}

pub struct CursorMovementCalculator;

impl CursorMovementCalculator {
    pub fn calculate_cursor_movement(new_sensitivity: i32, _dpi: i32) -> i32 {
        let sensitivity = 8.0;
        let movement = 3.0;
        let k = sensitivity * movement;
        let cursor_movement = k / (new_sensitivity as f64);
        cursor_movement.round() as i32
    }
}

/// Struct to auto-fetch game config and compute recoil/ADS values
pub struct RecoilSetup {
    pub fov: f64,
    pub sens: f64,
    pub xfactor: f64,
    pub x1modifier: f64,
    pub x15modifier: f64,
    pub x2modifier: f64,
    pub x25modifier: f64,
    pub x3modifier: f64,
    pub x4modifier: f64,
    pub dpi: i32,
    pub ads: HashMap<&'static str, i32>,
    pub ads_recoil: HashMap<&'static str, i32>,
}

impl RecoilSetup {
    pub fn auto_fetch() -> Option<Self> {
        // Try to find the latest GameSettings.ini in Documents\My Games\Rainbow Six - Siege\*\GameSettings.ini
        let doc_dir = dirs::document_dir()?;
        let r6_dir = doc_dir.join("My Games").join("Rainbow Six - Siege");
        let mut latest_ini: Option<PathBuf> = None;
        let mut latest_time = std::time::SystemTime::UNIX_EPOCH;
        if let Ok(entries) = fs::read_dir(&r6_dir) {
            for entry in entries.flatten() {
                let ini_path = entry.path().join("GameSettings.ini");
                if ini_path.exists() {
                    if let Ok(meta) = fs::metadata(&ini_path) {
                        if let Ok(modified) = meta.modified() {
                            if modified > latest_time {
                                latest_time = modified;
                                latest_ini = Some(ini_path);
                            }
                        }
                    }
                }
            }
        }
        let ini_path = latest_ini?;

        // Parse the ini file for required values
        let content = fs::read_to_string(&ini_path).ok()?;
        let mut fov = None;
        let mut sens = None;
        let mut xfactor = None;
        let mut x1 = None;
        let mut x15 = None;
        let mut x2 = None;
        let mut x25 = None;
        let mut x3 = None;
        let mut x4 = None;

        for line in content.lines() {
            let line = line.trim();
            if line.starts_with("DefaultFOV") {
                fov = line.split('=').nth(1).and_then(|v| v.trim().parse().ok());
            } else if line.starts_with("MouseYawSensitivity") {
                sens = line.split('=').nth(1).and_then(|v| v.trim().parse().ok());
            } else if line.starts_with("XFactorAiming") {
                xfactor = line.split('=').nth(1).and_then(|v| v.trim().parse().ok());
            } else if line.starts_with("ADSMouseSensitivity1x=") {
                x1 = line.split('=').nth(1).and_then(|v| v.trim().parse().ok());
            } else if line.starts_with("ADSMouseSensitivity1xHalf=") {
                x15 = line.split('=').nth(1).and_then(|v| v.trim().parse().ok());
            } else if line.starts_with("ADSMouseSensitivity2x=") {
                x2 = line.split('=').nth(1).and_then(|v| v.trim().parse().ok());
            } else if line.starts_with("ADSMouseSensitivity2xHalf=") {
                x25 = line.split('=').nth(1).and_then(|v| v.trim().parse().ok());
            } else if line.starts_with("ADSMouseSensitivity3x=") {
                x3 = line.split('=').nth(1).and_then(|v| v.trim().parse().ok());
            } else if line.starts_with("ADSMouseSensitivity4x=") {
                x4 = line.split('=').nth(1).and_then(|v| v.trim().parse().ok());
            }
        }

        // Prompt for DPI if not set (or use a default)
        let dpi = 800;

        let (fov, sens, xfactor, x1, x15, x2, x25, x3, x4) =
            (fov?, sens?, xfactor?, x1?, x15?, x2?, x25?, x3?, x4?);

        let calc = ScopeSensitivityCalculator::new(
            fov, sens, xfactor, x1, x15, x2, x25, x3, x4,
        );
        let ads = calc.calculate_ads_values();
        let mut ads_recoil = HashMap::new();
        for (k, v) in &ads {
            ads_recoil.insert(*k, CursorMovementCalculator::calculate_cursor_movement(*v, dpi));
        }

        Some(Self {
            fov,
            sens,
            xfactor,
            x1modifier: x1,
            x15modifier: x15,
            x2modifier: x2,
            x25modifier: x25,
            x3modifier: x3,
            x4modifier: x4,
            dpi,
            ads,
            ads_recoil,
        })
    }

    pub fn get_ads_options(&self) -> Vec<(&'static str, i32, i32)> {
        // Returns (name, ads_value, recoil_value)
        let mut out = Vec::new();
        for k in self.ads.keys() {
            out.push((
                *k,
                self.ads[k],
                *self.ads_recoil.get(k).unwrap_or(&0),
            ));
        }
        out
    }
}
