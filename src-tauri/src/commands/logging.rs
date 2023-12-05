use log::{error, info, warn};

#[tauri::command]
pub fn log_error(msg: &str) {
    error!("{msg}")
}

#[tauri::command]
pub fn log_warn(msg: &str) {
    warn!("{msg}")
}

#[tauri::command]
pub fn log_info(msg: &str) {
    info!("{msg}")
}
