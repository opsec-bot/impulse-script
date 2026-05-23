use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::sync::{ mpsc::{ channel, Sender, TryRecvError }, OnceLock };
use chrono::Local;

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

static LOG_TX: OnceLock<Sender<String>> = OnceLock::new();
static LOG_PATH: OnceLock<PathBuf> = OnceLock::new();

pub fn init_logger() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join("impulseDebug.txt");

    if LOG_PATH.get().is_none() {
        let _ = LOG_PATH.set(file_path.clone());
    }

    let (tx, rx) = channel::<String>();
    let _ = LOG_TX.set(tx);

    std::thread::spawn(move || {
        let file_result = OpenOptions::new().create(true).append(true).open(&file_path);

        let mut file = match file_result {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Failed to open log file: {}", e);
                return;
            }
        };

        while let Ok(line) = rx.recv() {
            let _ = file.write_all(line.as_bytes());
            #[cfg(debug_assertions)]
            print!("{}", line);

            loop {
                match rx.try_recv() {
                    Ok(more) => {
                        let _ = file.write_all(more.as_bytes());
                        #[cfg(debug_assertions)]
                        print!("{}", more);
                    }
                    Err(TryRecvError::Empty) => break,
                    Err(TryRecvError::Disconnected) => return,
                }
            }

            let _ = file.flush();
        }
    });

    log_debug("Logger initialized successfully");
    Ok(())
}

pub fn log_debug(message: &str) {
    log_with_level(LogLevel::Debug, message);
}

pub fn log_warning(message: &str) {
    log_with_level(LogLevel::Warning, message);
}

pub fn log_error(message: &str) {
    log_with_level(LogLevel::Error, message);
}

pub fn log_fatal(message: &str) {
    log_with_level(LogLevel::Fatal, message);
}

pub fn get_log_file_path() -> Option<PathBuf> {
    LOG_PATH.get().cloned()
}

fn log_with_level(level: LogLevel, message: &str) {
    let Some(sender) = LOG_TX.get() else {
        return;
    };

    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
    let log_line = format!("[{}][{}] {}\n", level.as_str(), timestamp, message);
    let _ = sender.send(log_line);
}
