use std::{ env, fs::{ self }, path::{ Path, PathBuf } };

use configparser::ini::Ini;
use glob::glob;
pub struct Setup {
    debug: bool,
    config: Ini,
    sensitivity_x: i32,
    sensitivity_y: i32,
    sensitivity_modifier_1: i32,
    sensitivity_modifier_15: i32,
    sensitivity_modifier_2: i32,
    sensitivity_modifier_25: i32,
    sensitivity_modifier_3: i32,
    sensitivity_modifier_4: i32,
    fov: i32,
    x_factor: f32,
    config_location: Option<PathBuf>,
    user_settings_path: PathBuf,
}

impl Setup {
    pub fn new(debug: bool) -> Self {
        let user_document_folder = Self::get_user_document_folder();
        let config_location = Self::get_game_settings_file(&user_document_folder);
        let appdata_dir = PathBuf::from(env::var("APPDATA").unwrap());
        let user_settings_path = appdata_dir.join("RCS");

        let config = Ini::new();

        Self {
            debug,
            config,
            sensitivity_x: 0,
            sensitivity_y: 0,
            sensitivity_modifier_1: 0,
            sensitivity_modifier_15: 0,
            sensitivity_modifier_2: 0,
            sensitivity_modifier_25: 0,
            sensitivity_modifier_3: 0,
            sensitivity_modifier_4: 0,
            fov: 0,
            x_factor: 0.0,
            config_location,
            user_settings_path,
        }
    }

    fn get_user_document_folder() -> PathBuf {
        dirs::document_dir().unwrap()
    }

    fn get_game_settings_file(user_document_folder: &Path) -> Option<PathBuf> {
        let r6_path = user_document_folder.join("My Games").join("Rainbow Six - Siege");
        let pattern = r6_path.join("*").join("GameSettings.ini");
        let pattern_str = pattern.to_string_lossy().replace("\\", "/");
        let mut ini_files: Vec<_> = glob(&pattern_str)
            .unwrap()
            .filter_map(Result::ok)
            .filter(|p| p.exists())
            .collect();


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
        } else if self.debug {
            eprintln!("No GameSettings.ini found in expected location.");
        }
    }

    pub fn debug_logging(&self) {
        if !self.debug {
            return;
        }
        println!(
            "Location: {:?}\nSENSITIVITY[X,Y]: ({}, {})\nFOV: {}\nxFactor: {}\n1x: {}\n1.5x: {}\n2x: {}\n2.5x: {}\n3x: {}\n4x: {}",
            self.config_location,
            self.sensitivity_x,
            self.sensitivity_y,
            self.fov,
            self.x_factor,
            self.sensitivity_modifier_1,
            self.sensitivity_modifier_15,
            self.sensitivity_modifier_2,
            self.sensitivity_modifier_25,
            self.sensitivity_modifier_3,
            self.sensitivity_modifier_4
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
}
