use crate::paths;
use reqwest::blocking::Client;
use uuid::Uuid;

use super::logging::log_info;

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

    let client = Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .map_err(|e| format!("Failed to build request client: {e}"))?;

    let response = client
        .get(url.to_string() + &name)
        .send()
        .and_then(|resp| resp.error_for_status())
        .map_err(|e| format!("Failed to send request: {e}"))?;

    log_info(&format!("Response: {:?}", response));

    let game_meta: Vec<GameMeta> = response
        .json()
        .map_err(|e| format!("Failed to parse response: {e}"))?;

    Ok(game_meta)
}

#[tauri::command]
pub fn download_image(url: String) -> Result<String, String> {
    let response = reqwest::blocking::get(url)
        .and_then(|resp| resp.error_for_status())
        .map_err(|err| format!("Failed to send request: {}", err))?;

    let image = response
        .bytes()
        .map_err(|err| format!("Failed to get image bytes: {}", err))?;

    let uuid = Uuid::new_v4();

    let image_path = paths::get_bpo()
        .join("images")
        .join(format!("{}.jpg", uuid.simple())); // Extension is hardcoded for now

    //  Write the image to the images folder and return the path
    std::fs::write(image_path.clone(), image)
        .map_err(|err| format!("Failed to write image: {}", err))?;

    Ok(image_path.to_str().unwrap().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::startup;

    fn setup() {
        startup::init();
    }

    #[test]
    fn test_download_img() {
        setup();

        let url = "https://picsum.photos/200";
        let image_path = download_image(url.to_string()).unwrap();

        assert!(image_path.contains("images"));
        assert!(image_path.contains(".jpg"));

        // cleanup
        std::fs::remove_file(image_path).unwrap();
        std::fs::remove_dir_all(paths::get_bpo()).unwrap();
    }
}
