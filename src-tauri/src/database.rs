use crate::{paths};

#[derive(serde::Serialize)]
pub struct Game {
    pub name: String,
    pub exe_path: String,
    pub playtime: f64,
}

pub fn build_game(name: String, exe_path: String, playtime: f64) -> Game {
    Game {
        name,
        exe_path,
        playtime,
    }
}

#[tauri::command]
pub fn save_to_db(title: String, exe_path: String) {
    // Establish a connection to the database file (library.db)
    let connection = sqlite::open(paths::get_pbp().join("library.db"))
        .expect("Crashed while connecting to database.");

    // Declare the query to execute in the sqlite file
    let query = format!(
        "INSERT INTO games VALUES ('{}', '{}', {});",
        title, exe_path, 0.0
    );

    // Execute the query
    connection
        .execute(query)
        .expect("Error while adding game to database.");
}

#[tauri::command]
pub fn get_from_db() -> Vec<Game> {
    // Establish a connection to the database file (library.db)
    let connection = sqlite::open(paths::get_pbp().join("library.db"))
        .expect("Crashed while connecting to database.");

    // Declare the query to execute in the sqlite file
    let query = "SELECT * FROM games";
    let mut result_vec = vec![];
    // Execute the query
    for row in connection
        .prepare(query)
        .expect("Preparing query failed")
        .into_iter()
        .map(|row| row.unwrap())
    {
        result_vec.push(build_game(
            row.read::<&str, _>("name").to_string(),
            row.read::<&str, _>("executable").to_string(),
            row.read::<f64, _>("hours"),
        ))
    }
    result_vec
}

#[tauri::command]
pub fn delete_from_db(name: String) {
    let connection = sqlite::open(paths::get_pbp().join("library.db"))
        .expect("Crashed while connecting to database");

    let query = format!(r#"DELETE FROM games WHERE name="{}";"#, name);
    connection
        .execute(query)
        .expect("Failed to execute database query");

    get_from_db();
}