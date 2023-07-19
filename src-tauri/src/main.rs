#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod paths;
mod plugins;
mod startup;

use tauri::api::process::Command;

fn main() {
    Command::new_sidecar("BPO-steam")
  .expect("failed to create `BPO-steam` binary command")
  .spawn()
  .expect("Failed to spawn sidecar");

    // Create the usual directories if they don't exist.
    startup::init();

    // This object is the initial tauri window
    // Tauri commands that can be called from the frontend are to be invoked below
    tauri::Builder::default()
        // Invoke your commands here
        .invoke_handler(tauri::generate_handler![
            commands::file_dialog,
            commands::image_dialog,
            commands::run_game,
            commands::logging::log,
            commands::database::save_to_db,
            commands::database::get_from_db,
            commands::database::edit_in_db,
            commands::database::delete_from_db,
            commands::database::wipe_library,
            commands::metadata::get_game_metadata,
            commands::metadata::download_image,
            plugins::scan_plugins,
            plugins::uninstall_plugin,
            plugins::search,
            steam_login
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_, _| {});
}

#[tauri::command]
async fn steam_login(handle: tauri::AppHandle) {
   tauri::WindowBuilder::new(
    &handle,
    "Steam",
    tauri::WindowUrl::External("http://localhost:5274/login".parse().unwrap())
  ).title("Steam Login").build().unwrap();
}