/*

    Project name: Project Black Pearl
    Date: Thursday, December 16th 2022
    Copyright holder: Project Black Pearl and it's contributors
    Copyright year: 2022

    This software is licensed under the BSD-3-Clause license.
    For more information -> https://opensource.org/licenses/BSD-3-Clause

*/

use crate::paths;
use rusqlite::{Connection, Result};
use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

fn setup_database(gamedb_path: &PathBuf) -> Result<()> {
    let mut conn = Connection::open(gamedb_path)?;
    let tx = conn.transaction()?;
    if tx
        .query_row(
            "SELECT name FROM sqlite_master WHERE type='table' AND name='schema_version'",
            [],
            //  Do nothing in the third argument because we want to only check if the table exists
            |_| Ok(()),
        )
        .is_err()
    {
        tx.execute(
            "CREATE TABLE schema_version (version INTEGER NOT NULL);",
            rusqlite::params![],
        )?;
        tx.execute(
            "INSERT INTO schema_version (version) VALUES (1)",
            rusqlite::params![],
        )?;
    }
    if tx
        .query_row(
            "SELECT name FROM sqlite_master WHERE type='table' AND name='games'",
            [],
            |_| Ok(()),
        )
        .is_err()
    {
        tx.execute(
            "CREATE TABLE IF NOT EXISTS games (\
                        id INTEGER PRIMARY KEY, \
                        name TEXT NOT NULL, \
                        executable TEXT NOT NULL, \
                        description TEXT, \
                        image TEXT);",
            rusqlite::params![],
        )?;
    } else {
        let mut statement: rusqlite::Statement =
            tx.prepare("SELECT \"notnull\" FROM PRAGMA_TABLE_INFO('games') WHERE name = 'name' OR name = 'executable'")?;
        let notnull = statement.query_map([], |row| row.get::<_, i32>(0))?;
        let mut name_notnull = false;
        let mut executable_notnull = false;
        for row in notnull {
            match row {
                Ok(1) => name_notnull = true,
                Ok(2) => executable_notnull = true,
                _ => (),
            }
        }
        if !name_notnull || !executable_notnull {
            tx.execute(
                "CREATE TABLE games_new (\
                            id INTEGER PRIMARY KEY, \
                            name TEXT NOT NULL, \
                            executable TEXT NOT NULL, \
                            description TEXT, \
                            image TEXT);",
                rusqlite::params![],
            )?;
            tx.execute(
                "INSERT INTO games_new (id, name, executable, description, image) \
                            SELECT id, name, executable, description, image FROM games;",
                rusqlite::params![],
            )?;
            tx.execute("DROP TABLE games", rusqlite::params![])?;
            tx.execute("ALTER TABLE games_new RENAME TO games", rusqlite::params![])?;
        }
    }
    tx.commit()?;
    Ok(())
}

pub fn init() {
    // Declare paths for directories and files inside of the PBP folder
    let pbp_path = paths::get_pbp();

    let temp_path = pbp_path.join("temp");
    let tempfile_path = temp_path.join("scrapers.json");

    let scraper_path = pbp_path.join("scrapers");
    let gamedb_path = pbp_path.join("library.db");
    let queries_path = pbp_path.join("queries");
    let images_path = pbp_path.join("images");
    let configfile_path = pbp_path.join("config.json");

    // Create the default directories if they don't exist
    if !pbp_path.exists() {
        create_folder(&pbp_path)
    }
    if !temp_path.exists() {
        create_folder(&temp_path)
    }
    if !queries_path.exists() {
        create_folder(&queries_path)
    }
    if !scraper_path.exists() {
        create_folder(&scraper_path)
    }
    if !images_path.exists() {
        create_folder(&images_path)
    }
    if !configfile_path.exists() {
        create_config(&configfile_path)
    }
    if !gamedb_path.exists() {
        match File::create(&gamedb_path) {
            Ok(_k) => {
                println!("Successfully created file {}", &gamedb_path.display());
            }
            Err(e) => {
                panic!("Error while creating config file: {}", e)
            }
        }
    }

    match setup_database(&gamedb_path) {
        Ok(_k) => {
            println!("Successfully created database {}", &gamedb_path.display());
        }
        Err(e) => {
            panic!("Error while creating database: {}", e)
        }
    }

    // If there are any temporary files created in the last instance of PBP, delete them.
    if tempfile_path.exists() {
        fs::remove_file(&tempfile_path).expect("Error while deleting temp file");
    }

    // Simplified function for creating directories
    fn create_folder(path: &Path) {
        match fs::create_dir_all(path) {
            Ok(k) => {
                println!("Successfully created folder {}", &path.display());
                k
            }
            Err(e) => eprintln!(
                "Error while creating folder {}: {}\n Your data will not be saved.",
                &path.display(),
                e
            ),
        }
    }

    // Creates the config file and writes to it
    fn create_config(path: &Path) {
        let mut file = match File::create(path) {
            Ok(k) => {
                println!("Successfully created file {}", &path.display());
                k
            }
            Err(e) => {
                panic!("Error while creating config file: {}", e)
            }
        };

        file.write_all(br#"{{ "currentLang": "en" }}"#)
            .expect("Failed to write to config file");
    }

    // Create a new file object
    // This will be the file where we save the scrapers that were found in the scrapers directory
    // I will refer to this file as "tempfile" from now on
    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(tempfile_path)
        .expect("Creating tempfile failed");

    // Read the directory 2 times
    // This is only a workaround for the rust borrow checker
    // This should be fixed sometime...
    let scan = fs::read_dir(&scraper_path).expect("Reading scrapers path failed");
    let scan_2 = fs::read_dir(&scraper_path).expect("Reading scrapers path failed");

    // This variable stores all of the scrapers paths found in the scrapers folder
    let entries: Vec<Result<fs::DirEntry, std::io::Error>> = scan_2.collect();
    println!("{} scrapers found.", entries.len());

    // Write this to the tempfile, this is the array which holds all of the entries
    file.write_all(r#"{ "scrapers": [ "#.as_bytes())
        .expect("Writing to tempfile failed");

    // Declare an iteration count
    let mut iter_count = 0;

    for entry in scan {
        // Handle the result of the entries
        // If there is an error, the function will return it
        let entry = entry.expect("Failed to handle scrapers directory entry");

        // Perform path to string conversion
        let entry_name = format!("{}", entry.file_name().to_string_lossy());
        let entry_location = format!("{}", entry.path().to_string_lossy());
        // Replace the single backslashes with 2 because it will otherwise escape the json string
        let entry_location = entry_location.replace('\\', r"\\");

        // If the files found in the scrapers directory end with .exe,
        // write their file name and path to the json file
        if entry_name.ends_with(".exe") {
            // Declare the string to write to the json file, it contains the file name and location of the scraper
            let json = format!(
                r#"{{ "name": "{}", "location": "{}" }}, "#,
                entry_name, entry_location
            );

            // Declare the line that should be written for the last scraper entry
            // This is the same as the first one, but without a trailing comma, that would lead to invalid json code
            let json_end = format!(
                r#"{{ "name": "{}", "location": "{}" }} "#,
                entry_name, entry_location
            );

            // Increase the iter count
            iter_count += 1;

            // If the iteration count is smaller than the amount of entries,
            // write the usual json string, otherwise, write the json_end string
            // It works like this because the last entry is not allowed to have a trailing comma
            let bytes = if iter_count < entries.len() {
                json.as_bytes()
            } else {
                json_end.as_bytes()
            };

            // Finally, write the bytes
            file.write_all(bytes)
                .expect("Writing to scraper tempfile failed");

            println!("Found scraper: {}", entry.file_name().to_string_lossy());
        }
    }

    // Close the array
    file.write_all(r#"] }"#.as_bytes())
        .expect("Writing to tempfile failed");
}
