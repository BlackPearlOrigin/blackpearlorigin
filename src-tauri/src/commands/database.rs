use crate::paths;
use rusqlite::{params, Connection};
use std::path::Path;
use uuid::Uuid;

use crate::commands::logging::log;

use super::logging::LogLevel;

#[derive(serde::Serialize)]
pub struct Game {
    pub id: i64,
    pub name: String,
    pub exe_path: String,
    pub description: String,
    pub image: String,
}

pub fn copy_image(image: &String) -> Result<std::path::PathBuf, std::io::Error> {
    let uuid = Uuid::new_v4();
    let uuid_simple = uuid.simple().to_string();

    log(
        LogLevel::Info,
        &format!("Generated the following (simple) UUID: {}", uuid_simple),
    );

    let mut image_path = Path::new("").to_path_buf();
    if !image.is_empty() {
        let image = Path::new(&image);
        image_path = paths::get_pbp().join("images").join(format!(
            "{}.{}",
            uuid_simple,
            image.extension().unwrap().to_str().unwrap()
        ));
        std::fs::copy(image, image_path.clone()).expect("Copying image failed");
        log(LogLevel::Info, "Copied image");
    };
    Ok(image_path)
}

#[tauri::command]
pub fn save_to_db(
    title: String,
    exe_path: String,
    description: String,
    image: String,
) -> Result<(), String> {
    // copy the image to the images folder
    let image_path = if image == "None" {
        log(LogLevel::Warning, "No image was copied since no image was provided");
        "None".to_string()
    } else {
        log(LogLevel::Info, "Copying image");
        copy_image(&image)
            .unwrap_or(Path::new("").to_path_buf())
            .display()
            .to_string()
    };

    // Establish a connection to the database file (library.db)
    let mut connection =
        Connection::open(paths::get_pbp().join("library.db")).map_err(|e| e.to_string())?;

    // Declare the query to execute in the sqlite file
    let query = "INSERT INTO games (name, executable, description, image) VALUES (?, ?, ?, ?)";

    let tx = connection.transaction().map_err(|e| e.to_string())?;

    tx.execute(query, params![title, exe_path, description, image_path])
        .map_err(|e| e.to_string())?;
    tx.commit().map_err(|e| e.to_string())?;

    log(LogLevel::Info, &format!("Saved game with name \"{}\" to the DB", title));
    Ok(())
}

#[tauri::command]
pub fn get_from_db() -> Result<Vec<Game>, String> {
    // Establish a connection to the database file (library.db)
    let connection =
        Connection::open(paths::get_pbp().join("library.db")).map_err(|e| e.to_string())?;

    // Declare the query to execute in the sqlite file
    let query = "SELECT * FROM games";
    let mut stmt = connection.prepare(query).map_err(|e| e.to_string())?;
    let mut rows = stmt.query([]).map_err(|e| e.to_string())?;

    let mut games = vec![];
    while let Ok(Some(row)) = rows.next() {
        let row = row;
        let id: i64 = row.get(0).map_err(|e| e.to_string())?;
        let name: String = row.get(1).map_err(|e| e.to_string())?;
        let exe_path: String = row.get(2).map_err(|e| e.to_string())?;
        let description: String = row.get(3).map_err(|e| e.to_string())?;
        let image: String = row.get(4).map_err(|e| e.to_string())?;

        games.push(Game {
            id,
            name,
            exe_path,
            description,
            image,
        });
    }

    log(LogLevel::Info, &format!("Got {} game(s) from DB", games.len()));
    Ok(games)
}

#[tauri::command]
pub fn edit_in_db(
    id: i64,
    name: String,
    executable: String,
    description: String,
    image: String,
) -> Result<(), String> {
    let mut connection =
        Connection::open(paths::get_pbp().join("library.db")).map_err(|e| e.to_string())?;
    // copy new image to location
    let image_path = if image == "None" {
        log(LogLevel::Warning, "No image was copied since no image was provided");
        "None".to_string()
    } else {
        log(LogLevel::Info, "Copying image");
        copy_image(&image)
            .unwrap_or(Path::new("").to_path_buf())
            .display()
            .to_string()
    };

    let query =
        "UPDATE games SET name = ?, executable = ?, description = ?, image = ? WHERE id = ?";

    let tx = connection.transaction().map_err(|e| e.to_string())?;
    tx.execute(
        query,
        params![name, executable, description, image_path, id],
    )
    .map_err(|e| e.to_string())?;
    tx.commit().map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn delete_from_db(id: i64) -> Result<(), String> {
    let mut connection =
        Connection::open(paths::get_pbp().join("library.db")).map_err(|e| e.to_string())?;

    let query = "DELETE FROM games WHERE id = ?;";
    let tx = connection.transaction().map_err(|e| e.to_string())?;
    tx.execute(query, params![id]).map_err(|e| e.to_string())?;
    tx.commit().map_err(|e| e.to_string())?;

    log(LogLevel::Info, &format!("Deleted game with id: {}", id));
    Ok(())
}

#[tauri::command]
pub fn wipe_library() -> Result<(), String> {
    let mut connection =
        Connection::open(paths::get_pbp().join("library.db")).map_err(|e| e.to_string())?;
    let query = "DELETE FROM games;";
    let tx = connection.transaction().map_err(|e| e.to_string())?;
    tx.execute(query, []).map_err(|e| e.to_string())?;
    tx.commit().map_err(|e| e.to_string())?;

    log(LogLevel::Info, "Wiped the entire library");
    Ok(())
}
