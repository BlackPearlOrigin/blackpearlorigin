use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum LogLevel {
    Error = 0,
    Warning = 1,
    Info = 2,
}

#[tauri::command]
pub fn log(level: LogLevel, log_message: &str) {

    let log_level = match level {
        LogLevel::Error => "ERROR",
        LogLevel::Warning => "WARNING",
        LogLevel::Info => "INFO",
    };

    println!("[{}] {}", log_level, log_message);
}
