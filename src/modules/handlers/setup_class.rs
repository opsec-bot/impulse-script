use std::{ collections::HashMap, env, fs::{ self, File }, path::{ Path, PathBuf } };
use std::io::Write;

use configparser::ini::Ini;
use glob::glob;
#[allow(dead_code)]
pub struct Setup {
    debug: bool,
    config: Ini,
    sensitivity_x: i32,
    sensitivity_y: i32,
    sensitivity: i32,
    sensitivity_modifier_1: i32,
    sensitivity_modifier_15: i32,
    sensitivity_modifier_2: i32,
    sensitivity_modifier_25: i32,
    sensitivity_modifier_3: i32,
    sensitivity_modifier_4: i32,
    recoil_x_value: i32,
    pub(crate) dpi: i32,
    fov: i32,
    x_factor: f32,
    ads: HashMap<String, f32>,
    ads_recoil: [i32; 6],
    user_document_folder: PathBuf,
    config_location: Option<PathBuf>,
    appdata_dir: PathBuf,
    user_settings_path: PathBuf,
    first_launch: bool,
}
#[allow(dead_code)]
impl Setup {
    pub fn new(debug: bool) -> Self {
        let user_document_folder = Self::get_user_document_folder();
        let config_location = Self::get_game_settings_file(&user_document_folder);
        let appdata_dir = PathBuf::from(env::var("APPDATA").unwrap());
        let user_settings_path = appdata_dir.join("RCS");
        let first_launch = Self::check_first_launch(&user_settings_path);

        let mut config = Ini::new();
        // If first launch, load the just-written default ini so config is populated immediately
        let user_ini_path = user_settings_path.join("user.ini");
        if user_ini_path.exists() {
            let _ = config.load(&user_ini_path);
        }

        Self {
            debug,
            config,
            sensitivity_x: 0,
            sensitivity_y: 0,
            sensitivity: 0,
            sensitivity_modifier_1: 0,
            sensitivity_modifier_15: 0,
            sensitivity_modifier_2: 0,
            sensitivity_modifier_25: 0,
            sensitivity_modifier_3: 0,
            sensitivity_modifier_4: 0,
            recoil_x_value: 0,
            dpi: 0,
            fov: 0,
            x_factor: 0.0,
            ads: HashMap::new(),
            ads_recoil: [0; 6],
            user_document_folder,
            config_location,
            appdata_dir,
            user_settings_path,
            first_launch,
        }
    }

    pub fn set_x_factor(&mut self, xmod: f32) {
        self.x_factor = xmod;
    }

    pub fn get_x_factor(&self) -> f32 {
        self.x_factor
    }

    pub fn get_dpi(&self) -> i32 {
        self.dpi
    }

    pub fn set_dpi(&mut self, dpi: i32) {
        self.dpi = dpi;
    }

    fn check_first_launch(user_settings_path: &Path) -> bool {
        if !user_settings_path.exists() {
            fs::create_dir_all(user_settings_path).unwrap();
        }
        let user_ini_path = user_settings_path.join("user.ini");
        if !user_ini_path.exists() {
            Self::write_default_ini(&user_ini_path);
            return true;
        }
        false
    }

    fn write_default_ini(path: &Path) {
        let default_content =
            r#"[RCS]
ingame_default = 90,7,58,146
ar_timings = {'416-C': 8, '552 COMMANDO': 9, '556XI': 9, 'AK-12': 7, 'AK-74M': 9, 'AR33': 8, 'ARX200': 9, 'AUG A2': 8, 'C7E': 8, 'C8-SFW': 7, 'F2': 6, 'G36C': 8, 'L85A2': 9, 'M4': 8, 'M762': 8, 'R4-C': 7, 'TYPE-89': 7, 'Test': 6}
smg_timings = {'9mm C1': 10, '9x19VSN': 8, 'AUG A3': 9, 'FMG-9': 8, 'K1A': 8, 'M12': 11, 'MP5': 8, 'MP5K': 8, 'MP5SD': 8, 'MP7': 7, 'MPX': 7, 'Mx4 Storm': 6, 'P10 RONI': 6, 'P90': 6, 'PDW9': 8, 'SCORPION EVO 3 A1': 6, 'T-5 SMG': 7, 'UMP45': 10, 'UZK50GI': 9, 'VECTOR .45 ACP': 5}
lmg_timings = {'6P41': 9, 'ALDA 5.56': 7, 'DP27': 11, 'G8A1': 7, 'LMG-E': 8, 'M249 SAW': 9, 'M249': 9, 'T-95 LSW': 9}
mp_timings = {'BEARING 9': 5, 'C75 Auto': 6, 'SMG-11': 5, 'SMG-12': 5, 'SPSMG9': 6, 'REAPER MK2': 6}
bind_panic = End
bind_toggle_menu = Ins
window_width = 500
window_height = 800
converted = True

[RCS_HOTKEY]
"#;
        let mut file = File::create(path).unwrap();
        file.write_all(default_content.as_bytes()).unwrap();
    }

    fn get_user_document_folder() -> PathBuf {
        dirs::document_dir().unwrap()
    }

    fn get_game_settings_file(user_document_folder: &Path) -> Option<PathBuf> {
        // Use \\ for Windows or / for cross-platform, glob crate supports /
        let r6_path = user_document_folder.join("My Games").join("Rainbow Six - Siege");
        let pattern = r6_path.join("*").join("GameSettings.ini");
        let pattern_str = pattern.to_string_lossy().replace("\\", "/"); // glob expects /
        let mut ini_files: Vec<_> = glob(&pattern_str)
            .unwrap()
            .filter_map(Result::ok)
            .filter(|p| p.exists())
            .collect();

        // Sort by modification time, newest last
        ini_files.sort_by_key(|p| {
            std::fs
                ::metadata(p)
                .and_then(|m| m.modified())
                .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
        });
        ini_files.pop()
    }

    pub fn get_mouse_sensitivity_settings(&mut self) {
        if let Some(ref config_path) = self.config_location {
            if let Err(e) = self.config.load(config_path) {
                if self.debug {
                    eprintln!("Failed to load GameSettings.ini: {:?}", e);
                }
                return;
            }

            let input = "INPUT";
            let display = "DISPLAY_SETTINGS";

            self.sensitivity_y = self.config
                .getint(input, "MouseYawSensitivity")
                .unwrap()
                .unwrap_or(0) as i32;
            self.sensitivity_x = self.config
                .getint(input, "MousePitchSensitivity")
                .unwrap()
                .unwrap_or(0) as i32;
            self.sensitivity_modifier_1 = self.config
                .getint(input, "ADSMouseSensitivity1x")
                .unwrap()
                .unwrap_or(0) as i32;
            self.sensitivity_modifier_15 = self.config
                .getint(input, "ADSMouseSensitivity1xHalf")
                .unwrap()
                .unwrap_or(0) as i32;
            self.sensitivity_modifier_2 = self.config
                .getint(input, "ADSMouseSensitivity2x")
                .unwrap()
                .unwrap_or(0) as i32;
            self.sensitivity_modifier_25 = self.config
                .getint(input, "ADSMouseSensitivity2xHalf")
                .unwrap()
                .unwrap_or(0) as i32;
            self.sensitivity_modifier_3 = self.config
                .getint(input, "ADSMouseSensitivity3x")
                .unwrap()
                .unwrap_or(0) as i32;
            self.sensitivity_modifier_4 = self.config
                .getint(input, "ADSMouseSensitivity4x")
                .unwrap()
                .unwrap_or(0) as i32;
            self.x_factor = self.config
                .getfloat(input, "XFactorAiming")
                .unwrap()
                .unwrap_or(0.0) as f32;
            self.fov = self.config.getfloat(display, "DefaultFOV").unwrap().unwrap_or(0.0) as i32;

            self.convert_for_recoil_calculation();
        } else if self.debug {
            eprintln!("No GameSettings.ini found in expected location.");
        }
    }

    pub fn convert_for_recoil_calculation(&mut self) {
        use crate::modules::handlers::ads_calc::{
            ScopeSensitivityCalculator,
            CursorMovementCalculator,
        };
        let ads_calculator = ScopeSensitivityCalculator::new(
            self.fov as f64,
            self.sensitivity_y as f64,
            self.x_factor as f64,
            self.sensitivity_modifier_1 as f64,
            self.sensitivity_modifier_15 as f64,
            self.sensitivity_modifier_2 as f64,
            self.sensitivity_modifier_25 as f64,
            self.sensitivity_modifier_3 as f64,
            self.sensitivity_modifier_4 as f64
        );
        let ads_values = ads_calculator.calculate_ads_values();
        let mut i = 0;
        let mut ads_recoil = [0; 6];
        for key in ["x1 ADS", "x15 ADS", "x2 ADS", "x25 ADS", "x3 ADS", "x4 ADS"] {
            if let Some(ads_val) = ads_values.get(key) {
                ads_recoil[i] = CursorMovementCalculator::calculate_cursor_movement(
                    *ads_val,
                    self.dpi
                );
                self.ads_recoil[i] = ads_recoil[i];
            }
            i += 1;
        }
        self.ads = ads_values
            .into_iter()
            .map(|(k, v)| (k, v as f32))
            .collect();
    }

    pub fn create_config_file(&self) {
        let mut user_config = Ini::new();
        user_config.set("USER", "DPI", Some(self.dpi.to_string()));
        let ads_str = self.ads_recoil
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",");
        user_config.set("USER", "ads_recoil", Some(ads_str));
        let config_path = self.user_settings_path.join("user.ini");
        user_config.write(config_path).unwrap();
    }

    pub fn debug_logging(&self) {
        if !self.debug {
            return;
        }
        println!(
            "Location: {:?}\nSensitivity: {}\nFirst Launch: {}\nSENSITIVITY[X,Y]: ({}, {})\nRecoil: {}\nDPI: {}\nFOV: {}\nxFactor: {}\n1x: {}\n1.5x: {}\n2x: {}\n2.5x: {}\n3x: {}\n4x: {}\nAds Recoil: {:?}",
            self.config_location,
            self.sensitivity,
            self.first_launch,
            self.sensitivity_x,
            self.sensitivity_y,
            self.recoil_x_value,
            self.dpi,
            self.fov,
            self.x_factor,
            self.sensitivity_modifier_1,
            self.sensitivity_modifier_15,
            self.sensitivity_modifier_2,
            self.sensitivity_modifier_25,
            self.sensitivity_modifier_3,
            self.sensitivity_modifier_4,
            self.ads_recoil
        );
        let _ = fs::remove_file(self.user_settings_path.join("user.ini"));
    }

    pub fn get_fov(&self) -> f32 {
        self.fov as f32
    }

    pub fn get_sensitivity(&self) -> f32 {
        self.sensitivity_y as f32
    }

    pub fn get_sensitivity_modifier_1(&self) -> f32 {
        self.sensitivity_modifier_1 as f32
    }

    pub fn get_sensitivity_modifier_25(&self) -> f32 {
        self.sensitivity_modifier_25 as f32
    }

    pub fn set_fov(&mut self, fov: i32) {
        self.fov = fov;
    }

    pub fn set_sensitivity(&mut self, sensitivity: i32) {
        self.sensitivity = sensitivity;
    }

    pub fn set_sensitivity_modifier_1(&mut self, modifier: i32) {
        self.sensitivity_modifier_1 = modifier;
    }

    pub fn set_sensitivity_modifier_25(&mut self, modifier: i32) {
        self.sensitivity_modifier_25 = modifier;
    }
}
