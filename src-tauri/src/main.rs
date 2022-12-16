/*

    Project name: Project Black Pearl
    Date: Thursday, Decenber 16th 2022
    Copyright holder: Project Black Pearl and it's contributors
    Copyright year: 2022

    This software is licensed under the BSD-3-Clause license.
    For more information -> https://opensource.org/licenses/BSD-3-Clause

*/

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod startup;
mod database;

#[tauri::command]
fn handle_scraper(scraper: u8) {
  match scraper {
    0 => {
        println!("Running binary scraper.");
    },
    1 => {
        println!("Running python scraper.");
    },
    _ => {
        // Invalid scraper type
    }
  }
}

fn main() {
    
    // Create the usual directories and look for scrapers.
    match startup::init() {
        Ok(k) => k,
        Err(e) => panic!("Failed to run initial startup: {}", e)
    }

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![handle_scraper])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
