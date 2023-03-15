use crate::commands::logging::log;
use crate::paths;
use reqwest::blocking::Client;
use uuid::Uuid;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct GameMeta {
    name: String,
    id: u32,
    cover_url: String,
    summary: String,
}

#[tauri::command]
pub fn get_game_metadata(name: String) -> Result<Vec<GameMeta>, String> {
    let url = "https://igdb-api.onrender.com/api/v1/game/";
    let client = match Client::builder().danger_accept_invalid_certs(true).build() {
        Ok(it) => it,
        Err(err) => panic!("Failed to build client: {}", err),
    };
    let response = match client.get(url.to_string() + &name).send() {
        Ok(it) => it,
        Err(err) => panic!("Failed to send request: {}", err),
    };
    log(2, format!("Response: {:?}", response).as_str());

    let game_meta: Vec<GameMeta> = match response.json() {
        Ok(it) => it,
        Err(err) => panic!("Failed to parse response: {}", err),
    };
    Ok(game_meta)
}

#[tauri::command]
pub fn download_image(url: String) -> Result<String, String> {
    let response = match reqwest::blocking::get(url) {
        Ok(it) => it,
        Err(err) => panic!("Failed to send request: {}", err),
    };
    let image = match response.bytes() {
        Ok(it) => it,
        Err(err) => panic!("Failed to parse response: {}", err),
    };
    let uuid = Uuid::new_v4();
    let image_path = paths::get_pbp()
        .join("images")
        .join(format!("{}.jpg", uuid.simple().to_string()));

    //  Write the image to the images folder and return the path
    match std::fs::write(image_path.clone(), image) {
        Ok(_) => Ok(image_path.display().to_string()),
        Err(err) => Err(err.to_string()),
    }
}
