pub mod rezi;
pub type ToFrontend = Vec<Vec<Item>>;

use std::thread;

use crate::commands::scrapers::rezi::rezi_scraper;

use super::logging::log_error;

#[derive(serde::Serialize)]
pub struct Item {
    scraper: String,
    name: String,
    links: Vec<String>
}

#[derive(serde::Deserialize)]
struct Query {
    query: String,
    scrapers: Vec<String>
}

#[tauri::command]
pub fn run_scraper(query: Query) {
    // todo: add the code
}