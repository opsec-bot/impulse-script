use std::path::Path;
use ini::Ini;

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
            let _ = std::fs::File::create(&self.file_name);
        }
    }

    pub fn read(&mut self) {
        self.config = Ini::load_from_file(&self.file_name).unwrap_or_else(|_| Ini::new());
    }

    pub fn sections(&self) -> Vec<String> {
        self.config
            .sections()
            .filter_map(|s| s.as_ref().map(|s| s.to_string()))
            .collect()
    }

    pub fn options(&self, section: &str) -> Vec<String> {
        self.config
            .section(Some(section))
            .map(|props| props.keys().cloned().collect())
            .unwrap_or_default()
    }

    pub fn get(&self, section: &str, option: &str) -> Option<String> {
        self.config
            .get_from(Some(section), option)
            .map(|v| v.to_string())
    }

    pub fn update(&mut self, section: &str, option: &str, value: impl ToString) {
        self.config
            .with_section(Some(section))
            .set(option, &value.to_string());
    }

    pub fn check_section_exist(&self, section: &str) -> bool {
        self.config.section(Some(section)).is_some()
    }

    pub fn delete_section(&mut self, section: &str) {
        self.config.delete(Some(section));
    }

    pub fn create_section(&mut self, section: &str) {
        if !self.check_section_exist(section) {
            self.config.with_section(Some(section));
        }
    }

    pub fn write(&self) {
        let _ = self.config.write_to_file(&self.file_name);
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
