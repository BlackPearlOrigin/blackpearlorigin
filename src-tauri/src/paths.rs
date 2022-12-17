/*

    Project name: Project Black Pearl
    Date: Thursday, December 16th 2022
    Copyright holder: Project Black Pearl and it's contributors
    Copyright year: 2022

    This software is licensed under the BSD-3-Clause license.
    For more information -> https://opensource.org/licenses/BSD-3-Clause

*/

use std::path::{Path, PathBuf};

pub fn get_pbp() -> PathBuf {

    // Get the username of the current user and define the local paths for Windows, MacOS and Linux

    let identifier = "org.blackpearl.PBP";
    let username = whoami::username();
    let pbp_path = match whoami::platform() {
        // Windows: C:\Users\username\AppData\Local\org.blackpearl.PBP
        whoami::Platform::Windows => Path::new(r"C:\")
            .join("Users")
            .join(username)
            .join("AppData")
            .join("Local")
            .join(format!("{}", identifier)),

        // Linux: /home/username/.local/share/org.blackpearl.PBP
        whoami::Platform::Linux => Path::new(r"/home")
            .join(username)
            .join(".local")
            .join("share")
            .join(format!("{}", identifier)),

        // macOS: /home/username/Library/Application Support/org.blackpearl.PBP
        whoami::Platform::MacOS => Path::new(r"/home")
            .join(username)
            .join("Library")
            .join("Application Support")
            .join(format!("{}", identifier)),

        _ => Path::new("").to_path_buf(),
    };

    // Return the corresponding path
    pbp_path
}
