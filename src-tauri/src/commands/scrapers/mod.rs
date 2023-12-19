pub mod rezi;
pub type ToFrontend = Vec<Vec<Item>>;

use crate::commands::scrapers::rezi::rezi_scraper;
use super::logging::log_error;
use rayon::prelude::*;

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

fn run_scraper(scraper_list: String) {
    let scraper_deser: Query = serde_json::from_str(&scraper_list).unwrap();

    let results = scraper_deser.scrapers
        .par_iter()
        .map(|scraper| {
            parse_scrapers(scraper.clone(), scraper_deser.query.clone())
        }); 
}

// todo: fix this code, my brain melted
fn parse_scrapers(scraper: String, query: String) -> Vec<Item> {
    match scraper.as_str() {
        "Rezi" => rezi_scraper(query),
        _ => {
            log_error("Scraper not found");
            Vec::new()
        },
    };
}