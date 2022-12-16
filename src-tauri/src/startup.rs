/*

    Project name: Project Black Pearl
    Date: Thursday, December 16th 2022
    Copyright holder: Project Black Pearl and it's contributors
    Copyright year: 2022

    This software is licensed under the BSD-3-Clause license.
    For more information -> https://opensource.org/licenses/BSD-3-Clause

*/

use std::{path::Path, fs, io::Write};
use crate::database::{self, connect};

pub fn init() -> std::io::Result<()> {

    let username = whoami::username();

    let local_path = match whoami::platform() {
        // Windows: C:\Users\username\AppData\Local\Project Black Pearl
        whoami::Platform::Windows => Path::new(r"C:\").join("Users").join(format!("{}", username)).join("AppData").join("Local").join("Project Black Pearl"),
        // Linux: /home/username/.local/share/Project Black Pearl
        whoami::Platform::Linux => Path::new(r"/home").join(format!("{}", username)).join(".local").join("share").join("Project Black Pearl"),
        // macOS: /home/username/Library/Application Support/Project Black Pearl
        whoami::Platform::MacOS => Path::new(r"/home").join(format!("{}", username)).join("Library").join("Application Support").join("Project Black Pearl"),
        _ => Path::new("").to_path_buf()
    };

    let temp_path = local_path.join("temp");
    let scraper_path = local_path.join("scrapers");
    let tempfile_path = temp_path.join("scrapers.json");
    let gamedb_path = local_path.join("library.db");

    if !local_path.exists() {
        create(&local_path)
    }
    if !temp_path.exists() {
        create(&temp_path)
    }
    if !scraper_path.exists() {
        create(&scraper_path)
    }
    if !gamedb_path.exists() {
        connect(&gamedb_path);
    }
    if tempfile_path.exists() {
        fs::remove_file(&tempfile_path).expect("Error while deleting temp file.");
    }

    fn create(path: &Path) {
        match fs::create_dir_all(&path) {
            Ok(k) => {
                println!("Successfully created folder {}", &path.display());
                k
            },
            Err(e) => eprintln!("Error while creating folder {}: {}\n Your data will not be saved.", &path.display(), e)
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

        if entry_name.ends_with(".exe") || entry_name.ends_with(".py") {

            let json = format!(r#"{{ "name": "{}", "location": "{}" }}, "#, entry_name, entry_location);
            let json_end = format!(r#"{{ "name": "{}", "location": "{}" }} "#, entry_name, entry_location);

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