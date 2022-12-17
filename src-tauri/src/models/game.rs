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
