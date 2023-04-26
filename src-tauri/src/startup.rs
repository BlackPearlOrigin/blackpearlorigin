/*

    Project name: Project Black Pearl
    Date: Thursday, December 16th 2022
    Copyright holder: Project Black Pearl and its contributors
    Copyright year: 2022

    This software is licensed under the BSD-3-Clause license.
    For more information -> https://opensource.org/licenses/BSD-3-Clause

*/

use crate::commands::logging::log;
use lazy_static::lazy_static;
use rusqlite::{Connection, Result};
use rusqlite_migration::{Migrations, M};
use std::{fs, io::Write, path};

// Define migrations. These are applied atomically.
lazy_static! {
    static ref MIGRATIONS: Migrations<'static> =
        Migrations::new(vec![
            M::up(include_str!("migrations/1_up.sql")).down(include_str!("migrations/1_down.sql")),
            // In the future, if the need to change the schema arises, put
            // migrations below.
        ]);
}

fn setup_database(gamedb_path: &path::PathBuf) -> Result<(), rusqlite_migration::Error> {
    let mut conn = Connection::open(gamedb_path)?;

    // Update the database schema, atomically
    MIGRATIONS.to_latest(&mut conn)
}

pub fn init() {
    // Declare paths for directories and files inside of the PBP folder

    // Folders
    let pbp_path = crate::paths::get_pbp();
    let plugin_path = pbp_path.join("plugin");
    let queries_path = pbp_path.join("queries");
    let images_path = pbp_path.join("images");
    let temp_path = pbp_path.join("temp");

    let gamedb_path = pbp_path.join("library.db");
    let configfile_path = pbp_path.join("config.json");

    let paths = [
        &pbp_path,
        &plugin_path,
        &queries_path,
        &images_path,
        &temp_path,
    ];

    // Create the default directories if they don't exist
    for path in paths {
        if !path.exists() {
            create_folder(path.as_path());
        }
    }

    if !configfile_path.exists() {
        let mut file = match fs::File::create(&configfile_path) {
            Ok(k) => {
                log(
                    2,
                    &format!("Successfully created file {}", &configfile_path.display()),
                );
                k
            }
            Err(e) => {
                panic!("[ERROR] Error while creating config file: {}", e)
            }
        };

        file.write_all(br#"{{ "currentLang": "en" }}"#)
            .expect("Failed to write to config file");
    }

    if !gamedb_path.exists() {
        match fs::File::create(&gamedb_path) {
            Ok(_) => {
                log(
                    2,
                    &format!("Successfully created file {}", &gamedb_path.display()),
                );
            }
            Err(e) => {
                panic!("[ERROR] Error while creating config file: {}", e)
            }
        }
    }

    match setup_database(&gamedb_path) {
        Ok(_) => {
            log(
                2,
                &format!("Successfully created database {}", &gamedb_path.display()),
            );
        }
        Err(e) => {
            panic!("[ERROR] Error while creating database: {}", e)
        }
    }

    // Simplified function for creating directories
    fn create_folder(path: &path::Path) {
        match fs::create_dir_all(path) {
            Ok(k) => {
                log(
                    2,
                    &format!("Successfully created folder {}", &path.display()),
                );
                k
            }
            Err(e) => eprintln!(
                "[ERROR] Error while creating folder {}: {}\n Your data will not be saved.",
                &path.display(),
                e
            ),
        }
    }

    log(2, "Welcome to Project Black Pearl")
}
