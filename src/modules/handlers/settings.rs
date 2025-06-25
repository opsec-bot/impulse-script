use std::path::Path;
use std::fs::File;
use configparser::ini::Ini;

#[derive(Debug)]
pub struct Settings {
    pub config: Ini,
    pub file_name: String,
}

impl Settings {
    pub fn new(file_name: &str) -> Self {
        Self {
            config: Ini::new(),
            file_name: file_name.to_string(),
        }
    }

    pub fn create(&self) {
        if !Path::new(&self.file_name).exists() {
            let _ = File::create(&self.file_name);
        }
    }

    pub fn read(&mut self) {
        let _ = self.config.load(&self.file_name);
    }

    pub fn sections(&self) -> Vec<String> {
        self.config.sections()
    }

    pub fn options(&self, section: &str) -> Vec<String> {
        self.config
            .get_map()
            .as_ref()
            .and_then(|map| map.get(section))
            .map(|props| props.keys().cloned().collect())
            .unwrap_or_default()
    }

    pub fn get(&self, section: &str, option: &str) -> Option<String> {
        self.config.get(section, option)
    }

    pub fn update(&mut self, section: &str, option: &str, value: impl ToString) {
        let value_str = value.to_string();
        self.config.set(section, option, Some(value_str));
    }

    pub fn check_section_exist(&self, section: &str) -> bool {
        self.config
            .get_map()
            .as_ref()
            .map(|map| map.contains_key(section))
            .unwrap_or(false)
    }

    pub fn create_section(&mut self, section: &str) {
        if !self.check_section_exist(section) {
            // configparser doesn't have a direct create_section, so set a dummy key
            self.config.set(section, "__dummy__", Some("".to_string()));
            // Remove the dummy key by setting it to None
            self.config.set(section, "__dummy__", None);
        }
    }

    pub fn write(&self) {
        let _ = self.config.write(&self.file_name);
    }

    pub fn check_updated(&self, section: &str, option: &str, value: &str) -> bool {
        self.get(section, option).as_deref() == Some(value)
    }

    pub fn comma_join(input: &[impl ToString]) -> String {
        input
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    pub fn parse(input: &str) -> Vec<String> {
        input
            .split(',')
            .map(|x| x.trim().to_string())
            .collect()
    }
}
