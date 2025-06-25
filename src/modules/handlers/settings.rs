use configparser::ini::Ini;

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

    pub fn read(&mut self) {
        let _ = self.config.load(&self.file_name);
    }

    pub fn sections(&self) -> Vec<String> {
        self.config.sections()
    }

    pub fn get(&self, section: &str, option: &str) -> Option<String> {
        self.config.get(section, option)
    }

    pub fn update(&mut self, section: &str, option: &str, value: impl ToString) {
        let value_str = value.to_string();
        self.config.set(section, option, Some(value_str));
    }

    pub fn write(&self) {
        let _ = self.config.write(&self.file_name);
    }
}
