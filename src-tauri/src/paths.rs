/*

    Project name: Black Pearl Origin
    Date: Thursday, December 16th 2022
    Copyright holder: Black Pearl Origin and its contributors
    Copyright year: 2022

    This software is licensed under the BSD-3-Clause license.
    For more information -> https://opensource.org/licenses/BSD-3-Clause

*/

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
