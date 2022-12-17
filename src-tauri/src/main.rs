/*

    Project name: Project Black Pearl
    Date: Thursday, December 16th 2022
    Copyright holder: Project Black Pearl and it's contributors
    Copyright year: 2022

    This software is licensed under the BSD-3-Clause license.
    For more information -> https://opensource.org/licenses/BSD-3-Clause

*/

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use execute::Execute;
use rfd::FileDialog;
use std::{path::Path, process::Command};

mod paths;
mod startup;

#[tauri::command]
fn handle_scraper(scraper: u8, path: String, query: String) {
    let path = Path::new(&path);
    let pbp_path = paths::get_pbp();

    // Support for different kinds of scrapers is to be implemented in the future.
    match scraper {
        0 => {
            println!(
                "Searching for {} with binary scraper: {}",
                query,
                path.display()
            );

            let mut command = Command::new(path);
            command.arg(query);
            command.arg(pbp_path.join("queries"));

            if let Some(exit_code) = command.execute().unwrap() {
                if exit_code == 0 {
                    println!("Scraper query completed successfully.");
                } else {
                    println!("Scraper query failed successfully.");
                }
            } else {
                println!("Scraper query interrupted.");
            }
        }
        _ => {
            // Invalid scraper type
        }
    }
}

#[tauri::command]
fn file_dialog() -> String {
    println!("Executable file dialog opened.");

    match FileDialog::new()
            .add_filter("Executables", &["exe", "com", "lnk", "cmd", "bat"])
            .set_directory("/")
            .pick_file()
        {
        Some(file) => {
            return file.display().to_string();
        }
        None => {
            return "None".to_string();
        }
    };
}

#[tauri::command]
fn save_to_db(title: String, exe_path: String) {
    let connection = sqlite::open(paths::get_pbp().join("library.db")).expect("Crashed while connecting to database.");
    let query = format!("INSERT INTO games VALUES ('{}', '{}', {});", title, exe_path, 0.0);
    connection.execute(query).expect("Error while adding game to database.");
}

fn main() {
    // Create the usual directories and look for scrapers.
    match startup::init() {
        Ok(k) => k,
        Err(e) => panic!("Failed to run initial startup: {}", e),
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            handle_scraper,
            file_dialog,
            save_to_db
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
