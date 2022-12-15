#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{path::Path, fs};

fn main() {
    
    create_paths();

    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn create_paths() {

    let username = whoami::username();

    let local_path = match whoami::platform() {
        whoami::Platform::Windows => Path::new(r"C:\").join("Users").join(format!("{}", username)).join("AppData").join("Local").join("Project Black Pearl"),
        whoami::Platform::Linux => Path::new(r"/home").join(format!("{}", username)).join(".local").join("Project Black Pearl"),
        whoami::Platform::MacOS => Path::new(r"/Users").join(format!("{}", username)).join(".local").join("Project Black Pearl"),
        _ => Path::new("").to_path_buf(),
    };

    let temp_folder_path = local_path.join("temp");

    if !local_path.exists() {
        create(&local_path)
    }
    if !temp_folder_path.exists() {
        create(&temp_folder_path)
    }

    fn create(path: &Path) {
        match fs::create_dir_all(&path) {
            Ok(k) => {
                println!("Successfully created folder {}", &path.display());
                k
            },
            Err(e) => eprintln!("Error while creating folder {}: {}\n Your data will not be saved.", &path.display(), e),
        }
    }
}

