use std::{process, thread, time::Instant};

use crate::commands::logging::log;
use rfd::FileDialog;

#[cfg(target_family = "unix")]
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::fs;

pub mod database;
pub mod logging;
pub mod metadata;

#[tauri::command]
// Opens a file dialog that prompts the user for an executable
pub fn file_dialog() -> String {
    log(2, "Executable file dialog opened".to_owned());

    // Prompt the user to select a file from their computer as an input
    // For error handling, you can use if- and match statements
    match FileDialog::new()
        .add_filter("Executables", &["exe", "com", "cmd", "bat", "sh"])
        .set_directory("/")
        .pick_file()
    {
        // If the user picked a file, return the path to the frontend
        Some(file) => file.display().to_string(),
        // If the user just closed the window, without picking a file, return "None" to the frontend
        None => "None".to_string(),
    }
}

#[tauri::command]
// Opens a file dialog that prompts the user for an image
pub fn image_dialog() -> String {
    log(2, "Image file dialog opened".to_owned());

    // Prompt the user to select a file from their computer as an input
    // For error handling, you can use if- and match statements
    match rfd::FileDialog::new()
        .add_filter(
            "Images",
            &["png", "jpg", "jpeg", "gif", "bmp", "ico", "webp"],
        )
        .pick_file()
    {
        // If the user picked a file, return the path to the frontend
        Some(file) => file.display().to_string(),
        // If the user just closed the window, without picking a file, return "None" to the frontend
        None => "None".to_string(),
    }
}

#[cfg(target_family = "unix")]
fn ensure_executable(target: PathBuf) {
    let perms = fs::Permissions::from_mode(0o770);
    fs::set_permissions(target, perms).unwrap();
}

#[tauri::command]
// This function is ran everytime the user clicks "Run" on a library entry
pub fn run_game(path: String) {
    let mut command = process::Command::new(&path);

    #[cfg(target_family = "unix")]
    ensure_executable(PathBuf::from(path));

    let start_time = Instant::now();

    thread::spawn(move || {
        let mut child = match command.spawn() {
            Ok(child) => child,
            Err(_) => return,
        };

        if let Ok(code) = child.wait() {
            if code.success() {
                let final_time = Instant::now() - start_time;
                log(
                    2,
                    format!("Game ran for {} second(s)", final_time.as_secs()),
                );
            } else {
                log(0, "Could not run game.".to_owned());
            }
        } else {
            println!("failed to wait for child process (wtf)");
        }
    });
}
