use crate::commands::logging::log;
use std::{path::PathBuf, process};
use tauri::api::path::local_data_dir;

pub fn get_bpo() -> PathBuf {
    let local_dir = match local_data_dir() {
        Some(dir) => dir,
        None => {
            log(0, "Failed to get local data dir.".to_owned());
            process::exit(0)
        }
    };

    let identifier = "io.github.blackpearlorigin";
    local_dir.join(identifier)
}

pub fn get_db() -> PathBuf {
    get_bpo().join("library.db")
}
