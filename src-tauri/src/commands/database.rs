use crate::paths;
use rusqlite::{params, Connection};
use std::{fs, path::PathBuf};
use uuid::Uuid;

use super::logging::log_info;

#[derive(serde::Serialize)]
pub struct Game {
    pub id: i64,
    pub name: String,
    pub exe_path: String,
    pub description: String,
    pub image: String,
}

pub fn copy_image(image_path: PathBuf) -> PathBuf {
    let uuid = Uuid::new_v4().simple().to_string();

    log_info(&format!("Generated the following (simple) UUID: {}", uuid));

    let file_name = match image_path.extension() {
        Some(extension) => format!("{}.{}", uuid, extension.to_string_lossy()),
        None => uuid,
    };

    let new_path = paths::get_bpo().join("images").join(file_name);
    fs::copy(image_path, &new_path).expect("Copying image failed");

    new_path
}

#[tauri::command]
pub fn save_to_db(
    title: String,
    exe_path: String,
    description: String,
    image: String,
) -> Result<(), String> {
    let image_path: PathBuf = if image.as_str() == "None" {
        "None".into()
    } else {
        copy_image(image.into())
    };

    // Establish a connection to the database file (library.db)
    let db_path = paths::get_db();
    let mut connection = Connection::open(db_path).map_err(|e| e.to_string())?;

    // Declare the query to execute in the sqlite file
    let query = "INSERT INTO games (name, executable, description, image) VALUES (?, ?, ?, ?)";
    let transaction = connection.transaction().map_err(|e| e.to_string())?;
    let params = params![title, exe_path, description, image_path.to_string_lossy()];

    transaction
        .execute(query, params)
        .map_err(|e| e.to_string())?;

    transaction.commit().map_err(|e| e.to_string())?;

    log_info(&format!("Saved game with name \"{}\" to the DB", title));
    Ok(())
}

#[tauri::command]
pub fn get_from_db() -> Result<Vec<Game>, String> {
    // Establish a connection to the database file (library.db)
    let db_path = paths::get_db();
    let connection = Connection::open(db_path).map_err(|e| e.to_string())?;

    // Declare the query to execute in the sqlite file
    let query = "SELECT * FROM games";
    let mut stmt = connection.prepare(query).map_err(|e| e.to_string())?;
    let mut rows = stmt.query([]).map_err(|e| e.to_string())?;

    let mut games = vec![];
    while let Ok(Some(row)) = rows.next() {
        games.push(Game {
            id: row.get(0).map_err(|e| e.to_string())?,
            name: row.get(1).map_err(|e| e.to_string())?,
            exe_path: row.get(2).map_err(|e| e.to_string())?,
            description: row.get(3).map_err(|e| e.to_string())?,
            image: row.get(4).map_err(|e| e.to_string())?,
        });
    }

    log_info(&format!("Got {} game(s) from DB", games.len()));
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
    let db_path = paths::get_bpo().join("library.db");
    let mut connection = Connection::open(db_path).map_err(|e| e.to_string())?;

    // copy new image to location
    let image_path: PathBuf = if image.as_str() == "None" {
        "None".into()
    } else {
        copy_image(image.into())
    };

    let query =
        "UPDATE games SET name = ?, executable = ?, description = ?, image = ? WHERE id = ?";
    let transaction = connection.transaction().map_err(|e| e.to_string())?;
    let params = params![
        name,
        executable,
        description,
        image_path.to_string_lossy(),
        id
    ];

    transaction
        .execute(query, params)
        .map_err(|e| e.to_string())?;

    transaction.commit().map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn delete_from_db(id: i64) -> Result<(), String> {
    let db_path = paths::get_bpo().join("library.db");
    let mut connection = Connection::open(db_path).map_err(|e| e.to_string())?;

    let query = "DELETE FROM games WHERE id = ?;";
    let tx = connection.transaction().map_err(|e| e.to_string())?;
    tx.execute(query, params![id]).map_err(|e| e.to_string())?;
    tx.commit().map_err(|e| e.to_string())?;

    log_info(&format!("Deleted game with id: {}", id));
    Ok(())
}

#[tauri::command]
pub fn wipe_library() -> Result<(), String> {
    let db_path = paths::get_bpo().join("library.db");
    let mut connection = Connection::open(db_path).map_err(|e| e.to_string())?;

    let query = "DELETE FROM games;";
    let tx = connection.transaction().map_err(|e| e.to_string())?;
    tx.execute(query, []).map_err(|e| e.to_string())?;
    tx.commit().map_err(|e| e.to_string())?;

    log_info("Wiped the entire library");
    Ok(())
}
