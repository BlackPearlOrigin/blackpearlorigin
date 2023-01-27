/*

    Project name: Project Black Pearl
    Date: Thursday, December 16th 2022
    Copyright holder: Project Black Pearl and its contributors
    Copyright year: 2022

    This software is licensed under the BSD-3-Clause license.
    For more information -> https://opensource.org/licenses/BSD-3-Clause

*/

use std::path;

pub fn get_pbp() -> path::PathBuf {
    let identifier = "org.blackpearl.PBP";
    let local_dir =
        &tauri::api::path::local_data_dir().expect("Failed to get local data directory.");

    // Return the PBP directory
    local_dir.join(identifier)
}
