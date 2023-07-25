use std::{process, thread, time::Instant};

use crate::{commands::logging::log, paths::get_bpo};
use rfd::FileDialog;

use std::fs;
#[cfg(target_family = "unix")]
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

pub mod database;
pub mod logging;
pub mod metadata;

#[tauri::command]
// Opens a file dialog that prompts the user for an executable
pub fn file_dialog() -> i32 {
    log(2, "Executable file dialog opened".to_owned());

    // Prompt the user to select a file from their computer as an input
    // For error handling, you can use if- and match statements
    let file = match FileDialog::new()
        .add_filter("7z archive", &["7z"])
        .set_directory("/")
        .pick_file()
    {
        // If the user picked a file, return the path to the frontend
        Some(file) => file.display().to_string(),
        // If the user just closed the window, without picking a file, return "None" to the frontend
        None => "None".to_string(),
    };

    // Creates a dir for the plugin
    let zip_name = PathBuf::from(file.clone());
    let plugin_path = get_bpo().join("plugins").join(zip_name.file_stem().unwrap()); 

    if !plugin_path.exists() {
        fs::create_dir(&plugin_path).expect("failed to create dir");
    
        // holy shit how didn't i think of this
        sevenz_rust::decompress_file(file, plugin_path).expect("complete");
    
        0
    } else {
        log(0, "Folder already exists, can't extract plugin".to_owned());
        
        1
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
    let mut command = process::Command::new(path.clone());

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
