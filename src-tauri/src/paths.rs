use std::{path::PathBuf, process};
use tauri::api::path::local_data_dir;

use crate::commands::logging::log_error;

pub fn get_bpo() -> PathBuf {
    let local_dir = match local_data_dir() {
        Some(dir) => dir,
        None => {
            log_error("Failed to get local data dir");
            process::exit(0)
        }
    };

    let identifier = "io.github.blackpearlorigin";
    local_dir.join(identifier)
}

pub fn get_db() -> PathBuf {
    get_bpo().join("library.db")
}
