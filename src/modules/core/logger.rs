use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;
use chrono::{ DateTime, Local };

pub enum LogLevel {
    Debug,
    Warning,
    Error,
    Fatal,
}

impl LogLevel {
    fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Debug => "Debug",
            LogLevel::Warning => "Warning",
            LogLevel::Error => "Error",
            LogLevel::Fatal => "Fatal",
        }
    }
}

pub struct Logger {
    file_path: PathBuf,
    file_handle: Mutex<Option<std::fs::File>>,
}

impl Logger {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let temp_dir = std::env::temp_dir();
        let file_path = temp_dir.join("impulseDebug.txt");

        let file = OpenOptions::new().create(true).append(true).open(&file_path)?;

        Ok(Self {
            file_path,
            file_handle: Mutex::new(Some(file)),
        })
    }

    pub fn log(&self, level: LogLevel, message: &str) {
        let timestamp: DateTime<Local> = Local::now();
        let formatted_time = timestamp.format("%Y-%m-%d %H:%M:%S%.3f");
        let log_line = format!("[{}][{}] {}\n", level.as_str(), formatted_time, message);

        if let Ok(mut file_guard) = self.file_handle.lock() {
            if let Some(ref mut file) = *file_guard {
                let _ = file.write_all(log_line.as_bytes());
                let _ = file.flush();
            }
        }

        print!("{}", log_line);
    }

    pub fn debug(&self, message: &str) {
        self.log(LogLevel::Debug, message);
    }

    pub fn warning(&self, message: &str) {
        self.log(LogLevel::Warning, message);
    }

    pub fn error(&self, message: &str) {
        self.log(LogLevel::Error, message);
    }

    pub fn fatal(&self, message: &str) {
        self.log(LogLevel::Fatal, message);
    }

    pub fn get_file_path(&self) -> &PathBuf {
        &self.file_path
    }
}

use std::sync::Once;
static GLOBAL_LOGGER: Mutex<Option<Logger>> = Mutex::new(None);
static INIT: Once = Once::new();

pub fn init_logger() -> Result<(), Box<dyn std::error::Error>> {
    INIT.call_once(|| {
        match Logger::new() {
            Ok(logger) => {
                logger.debug("Logger initialized successfully");
                let mut global_logger = GLOBAL_LOGGER.lock().unwrap();
                *global_logger = Some(logger);
            }
            Err(e) => {
                eprintln!("Failed to initialize logger: {}", e);
            }
        }
    });
    Ok(())
}

pub fn log_debug(message: &str) {
    if let Some(ref logger) = *GLOBAL_LOGGER.lock().unwrap() {
        logger.debug(message);
    }
}

pub fn log_warning(message: &str) {
    if let Some(ref logger) = *GLOBAL_LOGGER.lock().unwrap() {
        logger.warning(message);
    }
}

pub fn log_error(message: &str) {
    if let Some(ref logger) = *GLOBAL_LOGGER.lock().unwrap() {
        logger.error(message);
    }
}

pub fn log_fatal(message: &str) {
    if let Some(ref logger) = *GLOBAL_LOGGER.lock().unwrap() {
        logger.fatal(message);
    }
}

pub fn get_log_file_path() -> Option<PathBuf> {
    GLOBAL_LOGGER.lock()
        .unwrap()
        .as_ref()
        .map(|logger| logger.get_file_path().clone())
}
