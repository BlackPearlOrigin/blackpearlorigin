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

mod models {
    pub mod game;
}

use execute::Execute;
use models::game::build_game;
use models::game::Game;
use rfd::FileDialog;
use std::thread;
use std::{path::Path, process::Command, vec};
use tauri::CustomMenuItem;
use tauri::SystemTray;
use tauri::SystemTrayMenu;

mod paths;
mod startup;

#[tauri::command]
fn handle_scraper(path: String, query: String) {
    // String to path conversion
    let path = Path::new(&path);

    println!(
        "Searching for {} with binary scraper: {}",
        query,
        path.display()
    );

    // Create a command object for the scraper chosen (The command is just it's path)
    // Pass in it's path, a query and the destination folder for the cache file as arguments
    let mut command = Command::new(path);
    command.arg(query);
    command.arg(paths::get_pbp().join("queries"));

    // Run the scraper and tell us about it's exit code
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

#[tauri::command]
fn file_dialog() -> String {
    println!("Executable file dialog opened.");

    // Prompt the user to select a file from their computer as an input
    // For error handling, you can use if- and match statements
    match FileDialog::new()
        .add_filter("Executables", &["exe", "com", "lnk", "cmd", "bat"])
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
fn save_to_db(title: String, exe_path: String) {
    // Establish a connection to the database file (library.db)
    let connection = sqlite::open(paths::get_pbp().join("library.db"))
        .expect("Crashed while connecting to database.");

    // Declare the query to execute in the sqlite file
    let query = format!(
        "INSERT INTO games VALUES ('{}', '{}', {});",
        title, exe_path, 0.0
    );

    // Execute the query
    connection
        .execute(query)
        .expect("Error while adding game to database.");
}

#[tauri::command]
fn get_from_db() -> Vec<Game> {
    // Establish a connection to the database file (library.db)
    let connection = sqlite::open(paths::get_pbp().join("library.db"))
        .expect("Crashed while connecting to database.");

    // Declare the query to execute in the sqlite file
    let query = "SELECT * FROM games";
    let mut result_vec = vec![];
    // Execute the query
    for row in connection
        .prepare(query)
        .expect("Preparing query failed")
        .into_iter()
        .map(|row| row.unwrap())
    {
        result_vec.push(build_game(
            row.read::<&str, _>("name").to_string(),
            row.read::<&str, _>("executable").to_string(),
            row.read::<f64, _>("hours"),
        ))
    }
    result_vec
}

#[tauri::command]
fn delete_from_db(name: String) {
    let connection = sqlite::open(paths::get_pbp().join("library.db"))
        .expect("Crashed while connecting to database");

    let query = format!(r#"DELETE FROM games WHERE name="{}";"#, name);
    connection
        .execute(query)
        .expect("Failed to execute database query");

    get_from_db();
}

#[tauri::command]
fn run_game(path: String) {
    // String to path conversion
    let path = Path::new(&path);

    let mut command = Command::new(path);
    thread::spawn(move || {
        command.execute().expect("Failed to run game");
    });
}

fn main() {
    // Create the usual directories and look for scrapers.
    startup::init();

    // Create the system tray icon
    let tray = SystemTray::new()
        .with_menu(SystemTrayMenu::new().add_item(CustomMenuItem::new("quit", "Quit")));

    // This object is the initial tauri window
    // Tauri commands that can be called from the frontend are to be invoked below
    tauri::Builder::default()
        // Add the system tray to the tauri object and handle it's events
        .system_tray(tray)
        .on_system_tray_event(|_app, event| match event {
            tauri::SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => std::process::exit(0),
                _ => {}
            },
            _ => {}
        })
        // Invoke your commands here
        .invoke_handler(tauri::generate_handler![
            handle_scraper,
            file_dialog,
            save_to_db,
            get_from_db,
            delete_from_db,
            run_game
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        // If you close the window, it won't be terminated, but minimized to your system tray
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}
