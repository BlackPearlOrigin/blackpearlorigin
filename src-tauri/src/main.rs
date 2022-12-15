#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::{path::Path, fs, io::Write};


fn main() {
    
    match initial_startup() {
        Ok(k) => k,
        Err(e) => panic!("Failed to run initial startup: {}", e)
    }

    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn initial_startup() -> std::io::Result<()> {

    let username = whoami::username();

    let local_path = match whoami::platform() {
        whoami::Platform::Windows => Path::new(r"C:\").join("Users").join(format!("{}", username)).join("AppData").join("Local").join("Project Black Pearl"),
        whoami::Platform::Linux => Path::new(r"/home").join(format!("{}", username)).join(".local").join("Project Black Pearl"),
        whoami::Platform::MacOS => Path::new(r"/Users").join(format!("{}", username)).join(".local").join("Project Black Pearl"),
        _ => Path::new("").to_path_buf()
    };

    let temp_path = local_path.join("temp");
    let scraper_path = local_path.join("scrapers");
    let tempfile_path = temp_path.join("scrapers.json");

    if !local_path.exists() {
        create(&local_path)
    }
    if !temp_path.exists() {
        create(&temp_path)
    }
    if !scraper_path.exists() {
        create(&scraper_path)
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
    println!("{}", entry_len);

    file.write(format!(r#"{{ "scrapers": [ "#).as_bytes())?;

    let mut iter_count = 0;

    for entry in scan {
        let entry = entry?;

        let entry_name = format!("{}", entry.file_name().to_string_lossy());
        let entry_location = format!("{}", entry.path().to_string_lossy());
        let entry_location = entry_location.replace(r#"\"#, r#"\\"#);

        let json = format!(r#"{{ "name": "{}", "location": "{}" }}, "#, entry_name, entry_location);
        let json_end = format!(r#"{{ "name": "{}", "location": "{}" }} "#, entry_name, entry_location);

        iter_count += 1;

        let bytes = if iter_count < entry_len {
            json.as_bytes()
        } else {
            json_end.as_bytes()
        };

        file.write(bytes)?;

        if entry.file_name().to_str().expect("").ends_with(".exe") {
            println!("Found scraper: {}", entry.file_name().to_string_lossy());
        }
    }

    file.write(format!(r#"] }}"#).as_bytes())?;

    Ok(())
}

