#[derive(Debug)]
pub enum LogLevel {
    ERROR,
    WARNING,
    INFO
}

#[tauri::command]
pub fn log(log_level: LogLevel, log_message: &str) {
    println!("[{:#?}] {}", log_level, log_message);
}