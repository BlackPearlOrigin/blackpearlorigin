/*

    Project name: Project Black Pearl
    Date: Thursday, December 16th 2022
    Copyright holder: Project Black Pearl and it's contributors
    Copyright year: 2022

    This software is licensed under the BSD-3-Clause license.
    For more information -> https://opensource.org/licenses/BSD-3-Clause

*/

use crate::paths;
use std::{fs::{self, File}, io::Write, path::Path};

pub fn init() -> std::io::Result<()> {
    let pbp_path = paths::get_pbp();

    let temp_path = pbp_path.join("temp");
    let scraper_path = pbp_path.join("scrapers");
    let tempfile_path = temp_path.join("scrapers.json");
    let gamedb_path = pbp_path.join("library.db");
    let queries_path = pbp_path.join("queries");

    if !pbp_path.exists() {
        create(&pbp_path)
    }
    if !temp_path.exists() {
        create(&temp_path)
    }
    if !queries_path.exists() {
        create(&queries_path)
    }
    if !scraper_path.exists() {
        create(&scraper_path)
    }
    if !gamedb_path.exists() {
        File::create(&gamedb_path).expect("Failed to create database file");
        let connection = sqlite::open(&gamedb_path).expect("Connecting to new database failed");
        let query = "CREATE TABLE games (name TEXT, executable TEXT, hours FLOAT);";
        connection.execute(query).expect("Failed to setup database table");
    }
    if tempfile_path.exists() {
        fs::remove_file(&tempfile_path).expect("Error while deleting temp file");
    }

    fn create(path: &Path) {
        match fs::create_dir_all(&path) {
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

    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(tempfile_path)?;

    let scan = fs::read_dir(&scraper_path)?;
    let scan_2 = fs::read_dir(&scraper_path)?;

    let entries: Vec<Result<fs::DirEntry, std::io::Error>> = scan_2.collect();
    let entry_len = entries.len();
    println!("{} scrapers found.", entry_len);

    file.write(format!(r#"{{ "scrapers": [ "#).as_bytes())?;

    let mut iter_count = 0;

    for entry in scan {
        let entry = entry?;
        let entry_name = format!("{}", entry.file_name().to_string_lossy());
        let entry_location = format!("{}", entry.path().to_string_lossy());
        let entry_location = entry_location.replace(r#"\"#, r#"\\"#);

        if entry_name.ends_with(".exe") || entry_name.ends_with(".lua") {
            let json = format!(
                r#"{{ "name": "{}", "location": "{}" }}, "#,
                entry_name, entry_location
            );

            let json_end = format!(
                r#"{{ "name": "{}", "location": "{}" }} "#,
                entry_name, entry_location
            );

            iter_count += 1;

            let bytes = if iter_count < entry_len {
                json.as_bytes()
            } else {
                json_end.as_bytes()
            };

            file.write(bytes)?;

            println!("Found scraper: {}", entry.file_name().to_string_lossy());
        }
    }

    file.write(format!(r#"] }}"#).as_bytes())?;

    Ok(())
}
