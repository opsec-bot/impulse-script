use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::{self, Write},
    path::{Path, PathBuf},
};

use configparser::ini::Ini;
use glob::glob;

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
    dpi: i32,
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

impl Setup {
    pub fn new(debug: bool) -> Self {
        let user_document_folder = Self::get_user_document_folder();
        let config_location = Self::get_game_settings_file(&user_document_folder);
        let appdata_dir = PathBuf::from(env::var("APPDATA").unwrap());
        let user_settings_path = appdata_dir.join("HELPY");
        let first_launch = Self::check_first_launch(&user_settings_path);

        Self {
            debug,
            config: Ini::new(),
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

    fn check_first_launch(user_settings_path: &Path) -> bool {
        if !user_settings_path.exists() {
            fs::create_dir_all(user_settings_path).unwrap();
            return true;
        }
        let user_ini_path = user_settings_path.join("user.ini");
        if !user_ini_path.exists() {
            File::create(user_ini_path).unwrap();
            return true;
        }
        false
    }

    fn get_user_document_folder() -> PathBuf {
        dirs::document_dir().unwrap()
    }

    fn get_game_settings_file(user_document_folder: &Path) -> Option<PathBuf> {
        let r6_path = user_document_folder.join("My Games/Rainbow Six - Siege");
        let pattern = r6_path.join("*/GameSettings.ini");
        let mut ini_files: Vec<_> = glob(pattern.to_str().unwrap())
            .unwrap()
            .filter_map(Result::ok)
            .collect();

        ini_files.sort_by_key(|p| fs::metadata(p).unwrap().modified().unwrap());
        ini_files.pop()
    }

    pub fn get_mouse_sensitivity_settings(&mut self) {
        if let Some(ref config_path) = self.config_location {
            self.config.load(config_path).unwrap();

            let input = "INPUT";
            let display = "DISPLAY_SETTINGS";

            self.sensitivity_y = self.config.getint(input, "MouseYawSensitivity").unwrap().unwrap_or(0);
            self.sensitivity_x = self.config.getint(input, "MousePitchSensitivity").unwrap().unwrap_or(0);
            self.sensitivity_modifier_1 = self.config.getint(input, "ADSMouseSensitivity1x").unwrap().unwrap_or(0);
            self.sensitivity_modifier_15 = self.config.getint(input, "ADSMouseSensitivity1xHalf").unwrap().unwrap_or(0);
            self.sensitivity_modifier_2 = self.config.getint(input, "ADSMouseSensitivity2x").unwrap().unwrap_or(0);
            self.sensitivity_modifier_25 = self.config.getint(input, "ADSMouseSensitivity2xHalf").unwrap().unwrap_or(0);
            self.sensitivity_modifier_3 = self.config.getint(input, "ADSMouseSensitivity3x").unwrap().unwrap_or(0);
            self.sensitivity_modifier_4 = self.config.getint(input, "ADSMouseSensitivity4x").unwrap().unwrap_or(0);
            self.x_factor = self.config.getfloat(input, "XFactorAiming").unwrap().unwrap_or(0.0);
            self.fov = self.config.getfloat(display, "DefaultFOV").unwrap().unwrap_or(0.0) as i32;
        }
    }

    pub fn create_config_file(&self) {
        let mut user_config = Ini::new();
        user_config.set("USER", "DPI", Some(&self.dpi.to_string()));
        let ads_str = self.ads_recoil.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(",");
        user_config.set("USER", "ads_recoil", Some(&ads_str));
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
}

fn main() {
    let mut setup = Setup::new(true);
    setup.dpi = 800;
    setup.get_mouse_sensitivity_settings();
    setup.create_config_file();
    setup.debug_logging();
}
