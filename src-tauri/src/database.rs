use crate::paths;
use std::path::Path;

#[derive(serde::Serialize)]
pub struct Game {
    pub name: String,
    pub exe_path: String,
    pub playtime: f64,
    pub description: String,
    pub image: String,
}

pub fn build_game(
    name: String,
    exe_path: String,
    playtime: f64,
    description: String,
    image: String,
) -> Game {
    Game {
        name,
        exe_path,
        playtime,
        description,
        image,
    }
}

#[tauri::command]
pub fn save_to_db(title: String, exe_path: String, description: String, image: String) {
    // copy the image to the images folder
    let mut image_path = Path::new("").to_path_buf();
    if image != "" {
        let image = Path::new(&image);
        image_path = paths::get_pbp()
            .join("images")
            .join(image.file_name().unwrap());
        std::fs::copy(image, image_path.clone()).expect("Copying image failed");
    };

    // Establish a connection to the database file (library.db)
    let connection = sqlite::open(paths::get_pbp().join("library.db"))
        .expect("Crashed while connecting to database.");

    // Declare the query to execute in the sqlite file
    let query = format!(
        "INSERT INTO games VALUES ('{}', '{}', {}, '{}', '{}');",
        title,
        exe_path,
        0.0,
        description,
        image_path.display().to_string()
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
            row.read::<&str, _>("description").to_string(),
            row.read::<&str, _>("image").to_string(),
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
