#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use commands::{logging::log_error, scrapers::rezi::search_rezi};

mod commands;
mod paths;
mod startup;

fn main() {
    env_logger::init();

    // Create the usual directories if they don't exist.
    startup::init();

    // example invokation of rezi scraper
    match search_rezi("starfield") {
        Ok(r) => println!("{r:#?}"),
        Err(e) => log_error(&e),
    }

    // This object is the initial tauri window
    // Tauri commands that can be called from the frontend are to be invoked below
    tauri::Builder::default()
        // Invoke your commands here
        .invoke_handler(tauri::generate_handler![
            commands::file_dialog,
            commands::image_dialog,
            commands::run_game,
            commands::logging::log_error,
            commands::logging::log_warn,
            commands::logging::log_info,
            commands::database::save_to_db,
            commands::database::get_from_db,
            commands::database::edit_in_db,
            commands::database::delete_from_db,
            commands::database::wipe_library,
            commands::metadata::get_game_metadata,
            commands::metadata::download_image,
            commands::scrapers::rezi::search_rezi,
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_, _| {});
}
