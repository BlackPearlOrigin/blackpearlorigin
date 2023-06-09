use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum LogLevel {
    Error = 0,
    Warning = 1,
    Info = 2,
}

#[tauri::command]
pub fn log(log_level: usize, log_message: String) {
    let log_levels = ["ERROR", "WARNING", "INFO"];
    println!("[{}] {}", log_levels[log_level], log_message);
}
