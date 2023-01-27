use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum LogLevel {
    ERROR = 0,
    WARNING = 1,
    INFO = 2,
}

#[tauri::command]
pub fn log(log_level: usize, log_message: &str) {
    let log_levels = ["ERROR", "WARNING", "INFO"];
    println!("[{}] {}", log_levels[log_level], log_message);
}
