pub mod rezi;
pub type ToFrontend = Vec<Vec<Item>>;

use crate::commands::scrapers::rezi::rezi_scraper;
use super::logging::log_error;
use rayon::prelude::*;

#[derive(serde::Serialize, Debug)]
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
pub async fn run_scraper(scrapers: Vec<String>, query: String) {
    let mut ret: Vec<Vec<Item>> = vec![];

    let results: Vec<Item> = scrapers
        .into_par_iter()
        .flat_map(async |scraper| {
            let parsed = parse_scrapers(scraper.clone(), query.clone());
            parsed.await.into_iter()
        })
        .collect();

    println!("{:?}", results); 
}

// todo: fix this code, my brain melted
async fn parse_scrapers(scraper: String, query: String) -> Vec<Item> {
    match scraper.as_str() {
        "rezi" => {
            let res = rezi_scraper(query);
            res.await
        },
        _ => {
            log_error("Scraper not found");
            Vec::new()
        }
    }
}