use std::{fs, path, process, thread, time::Instant};

use crate::commands::logging::log;
use execute::Execute;
use rfd::FileDialog;

pub mod database;
pub mod logging;

#[tauri::command]
// This function is ran everytime a search query is made
pub fn handle_scraper(path: String, query: String) {
    let start_time = Instant::now();

    // Create a command object for the scraper chosen (The command is just its path)
    // Pass in its path, a query and the destination folder for the cache file as arguments
    let mut command = process::Command::new(path.clone());
    command.arg(query.clone());
    command.arg(crate::paths::get_pbp().join("queries"));

    log(
        2,
        &format!("Searching for \"{}\" with {}", query, path),
    );

    // Run the scraper and tell us about its exit code
    if let Some(exit_code) = command.execute().unwrap() {
        if exit_code == 0 {
            log(2, "Scraper query completed successfully");
            let final_time = Instant::now() - start_time;
            log(
                2,
                &format!("Took {} second(s)", final_time.as_secs()),
            )
        } else {
            log(0, "Scraper query failed successfully");
        }
    } else {
        log(2, "Scraper query interrupted");
    }
}

#[tauri::command]
// This function is ran everytime the user clicks the "Install scraper" button on the Preferences page
pub fn install_scraper() {
    let file = match FileDialog::new()
        .add_filter("Executables", &["exe", "com", "cmd", "bat"])
        .set_directory("/")
        .pick_file()
    {
        Some(file) => file.display().to_string(),
        None => "None".to_string(),
    };

    // Copy the scraper from the location the user selected to the scrapers folder
    if file != "None" {
        let file = path::Path::new(&file);
        fs::copy(
            file,
            crate::paths::get_pbp()
                .join("scrapers")
                .join(file.file_name().unwrap()),
        )
        .expect("Installing scraper failed");
    }

    log(
        2,
        &format!(
            "Installed scraper with path {}",
            path::Path::new(&file).display().to_string()
        ),
    );
}

#[tauri::command]
// Opens a file dialog that prompts the user for an executable
pub fn file_dialog() -> String {
    log(2, "Executable file dialog opened");

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
    log(2, "Image file dialog opened");

    // Prompt the user to select a file from their computer as an input
    // For error handling, you can use if- and match statements
    match rfd::FileDialog::new()
        .add_filter(
            "Images",
            &["png", "jpg", "jpeg", "gif", "bmp", "ico", "webp"],
        )
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
// This function is ran everytime the user clicks "Run" on a library entry
pub fn run_game(path: String) {
    let start_time = Instant::now();
    let mut command = process::Command::new(path);

    thread::spawn(move || {
        if let Some(exit_code) = command.execute().unwrap() {
            if exit_code == 0 {
                log(2, "Game ran successfully");
                let final_time = Instant::now() - start_time;
                log(
                    2,
                    &format!("Game ran for {} second(s)", final_time.as_secs()),
                )
            } else {
                log(0, "Scraper query failed successfully");
            }
        };
    });
}
