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
    let username = whoami::username();
    let pbp_path = match whoami::platform() {
        // Windows: C:\Users\username\AppData\Local\Project Black Pearl
        whoami::Platform::Windows => Path::new(r"C:\")
            .join("Users")
            .join(format!("{}", username))
            .join("AppData")
            .join("Local")
            .join("Project Black Pearl"),

        // Linux: /home/username/.local/share/Project Black Pearl
        whoami::Platform::Linux => Path::new(r"/home")
            .join(format!("{}", username))
            .join(".local")
            .join("share")
            .join("Project Black Pearl"),

        // macOS: /home/username/Library/Application Support/Project Black Pearl
        whoami::Platform::MacOS => Path::new(r"/home")
            .join(format!("{}", username))
            .join("Library")
            .join("Application Support")
            .join("Project Black Pearl"),

        _ => Path::new("").to_path_buf(),
    };

    pbp_path
}
