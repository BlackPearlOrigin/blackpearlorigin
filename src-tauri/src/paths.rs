/*

    Project name: Black Pearl Origin
    Date: Thursday, December 16th 2022
    Copyright holder: Black Pearl Origin and its contributors
    Copyright year: 2022

    This software is licensed under the BSD-3-Clause license.
    For more information -> https://opensource.org/licenses/BSD-3-Clause

*/

use std::path;

pub fn get_bpo() -> path::PathBuf {
    let identifier = "io.github.blackpearlorigin";
    let local_dir =
        &tauri::api::path::local_data_dir().expect("Failed to get local data directory.");

    // Return the BPO directory
    local_dir.join(identifier)
}
