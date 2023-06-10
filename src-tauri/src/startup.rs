use crate::{commands::logging::log, paths::get_bpo};
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
    let bpo_path = get_bpo();

    // Create default folders
    let folders = vec!["plugins", "queries", "images"];
    for folder in folders {
        create_folder(&bpo_path.join(folder));
    }

    let gamedb_path = bpo_path.join("library.db");
    let configfile_path = bpo_path.join("config.json");

    if !configfile_path.exists() {
        let mut file = match fs::File::create(&configfile_path) {
            Ok(file) => {
                log(
                    2,
                    format!("Successfully created file {}", &configfile_path.display()),
                );
                file
            }
            Err(e) => {
                panic!("[ERROR] Error while creating config file: {}", e);
            }
        };

        if file.write_all(br#"{{ "currentLang": "en" }}"#).is_err() {
            log(0, "Failed to write config file!".to_owned());
        }
    }

    if !gamedb_path.exists() {
        if let Err(e) = fs::File::create(&gamedb_path) {
            panic!("[ERROR] Error while creating config file: {}", e);
        } else {
            log(
                2,
                format!("Successfully created file {}", &gamedb_path.display()),
            );
        }
    }

    if let Err(e) = setup_database(&gamedb_path) {
        panic!("[ERROR] Error while creating database: {}", e)
    } else {
        log(
            2,
            format!("Successfully created database {}", &gamedb_path.display()),
        );
    }

    // Simplified function for creating directories
    fn create_folder(path: &path::PathBuf) {
        if let Err(e) = fs::create_dir_all(path) {
            eprintln!(
                "[ERROR] Error while creating folder {}: {}",
                &path.display(),
                e
            );
            eprintln!("Your data may not be saved.");
        } else {
            log(2, format!("Created folder {}", &path.display()));
        }
    }

    println!("Welcome to Black Pearl Origin!")
}
