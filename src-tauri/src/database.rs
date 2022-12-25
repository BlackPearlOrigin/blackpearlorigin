use crate::paths;
use std::path::Path;
use uuid::Uuid;

#[derive(serde::Serialize)]
pub struct Game {
    pub id: i64,
    pub name: String,
    pub exe_path: String,
    pub playtime: f64,
    pub description: String,
    pub image: String,
}

pub fn build_game(
    id: i64,
    name: String,
    exe_path: String,
    playtime: f64,
    description: String,
    image: String,
) -> Game {
    Game {
        id,
        name,
        exe_path,
        playtime,
        description,
        image,
    }
}

pub fn copy_image(image: &String) -> Result<std::path::PathBuf, std::io::Error> {
    let uuid = Uuid::new_v4();
    let uuid_simple = uuid.simple().to_string();

    let mut image_path = Path::new("").to_path_buf();
    if !image.is_empty() {
        let image = Path::new(&image);

        if image.ends_with(".png")
        || image.ends_with(".jpg")
        || image.ends_with(".jpeg")
        || image.ends_with(".gif")
        || image.ends_with(".bmp")
        || image.ends_with(".ico")
        || image.ends_with(".webp")
        {
            image_path = paths::get_pbp().join("images").join(format!(
                "{}.{}",
                uuid_simple,
                image.extension().unwrap().to_str().unwrap()
            ));
            std::fs::copy(image, image_path.clone()).expect("Copying image failed");
        }
    };
    Ok(image_path)
}

#[tauri::command]
pub fn save_to_db(title: String, exe_path: String, description: String, image: String) {
    // copy the image to the images folder
    let image_path = copy_image(&image).expect("Copying image failed");

    // Establish a connection to the database file (library.db)
    let connection = sqlite::open(paths::get_pbp().join("library.db"))
        .expect("Crashed while connecting to database.");

    // Declare the query to execute in the sqlite file
    let query = "INSERT INTO games \
    (name, executable, hours, description, image) \
    VALUES (?, ?, ?, ?, ?);";
    let mut statement = connection.prepare(query).expect("Failed to prepare query");
    statement.bind((1, &*title)).expect("Failed to bind title");
    statement
        .bind((2, &*exe_path))
        .expect("Failed to bind exe_path");
    statement.bind((3, 0.0)).expect("Failed to bind playtime");
    statement
        .bind((4, &*description))
        .expect("Failed to bind description");
    statement
        .bind((5, &*(image_path.display().to_string())))
        .expect("Failed to bind image");

    // Execute the query
    while let Ok(sqlite::State::Row) = statement.next() {}
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
            row.read::<i64, _>("id"),
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
pub fn edit_in_db(id: i64, name: String, executable: String, description: String, image: String) {
    let connection = sqlite::open(paths::get_pbp().join("library.db"))
        .expect("Crashed while connecting to database");
    // copy new image to location
    let image_path = copy_image(&image).expect("Copying image failed");
    let query =
        "UPDATE games SET name = ?, executable = ?, description = ?, image = ? WHERE id = ?;";
    let mut statement = connection.prepare(query).expect("Failed to prepare query");
    statement.bind((1, &*name)).expect("Failed to bind name");
    statement
        .bind((2, &*executable))
        .expect("Failed to bind executable");
    statement
        .bind((3, &*description))
        .expect("Failed to bind description");
    statement
        .bind((4, &*image_path.display().to_string()))
        .expect("Failed to bind image");
    statement.bind((5, id)).expect("Failed to bind id");

    while let Ok(sqlite::State::Row) = statement.next() {}
}

#[tauri::command]
pub fn delete_from_db(id: i64) {
    let connection = sqlite::open(paths::get_pbp().join("library.db"))
        .expect("Crashed while connecting to database");

    let query = "DELETE FROM games WHERE id = ?;";
    let mut statement = connection.prepare(query).expect("Failed to prepare query");
    statement.bind((1, id)).expect("Failed to bind id");

    while let Ok(sqlite::State::Row) = statement.next() {}
}

#[tauri::command]
pub fn wipe_library() {
    let connection =
        sqlite::open(paths::get_pbp().join("library.db")).expect("Connecting to database failed");
    let query = "DELETE FROM games;";
    connection
        .execute(query)
        .expect("Executing database query failed");
}
