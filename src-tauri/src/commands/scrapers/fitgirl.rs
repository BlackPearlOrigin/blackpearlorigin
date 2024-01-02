use crate::commands::scrapers::rezi::Item;
use rayon::prelude::*;
use reqwest::header::CONTENT_TYPE;
use scraper::{Html, Selector};

use crate::commands::logging::log_error;

#[tauri::command]
pub fn search_fitgirl(query: &str) -> Option<Vec<Item>> {
    let url = format!("https://www.fitgirl-repacks.site?s={}", query);
    let client = reqwest::blocking::Client::new();
    let response = match client
        .get(url)
        .header(CONTENT_TYPE, "text/html")
        // Chrome on Windows UA
        .send()
        .map_err(|e| e.to_string())
    {
        Ok(r) => r,
        Err(e) => {
            log_error(&e);
            return None;
        }
    };

    let text = match response.text().map_err(|e| e.to_string()) {
        Ok(t) => t,
        Err(e) => {
            log_error(&e);
            return None;
        }
    };

    let document = Html::parse_document(&text);
    let a_selector = Selector::parse(".entry-title a").unwrap();
    let title_selector = Selector::parse(".entry-title").unwrap();

    // get all links and iter over them, making a new request, yada yada yada
    let links = document
        .select(&a_selector)
        .map(|element| element.value().attr("href").unwrap().to_string())
        .collect::<Vec<String>>();

    let items = links
        .par_iter()
        .map(|link| parse_link(link.to_string()).unwrap_or(String::new()))
        .collect::<Vec<String>>();

    let titles = document
        .select(&title_selector)
        .map(|element| element.inner_html())
        .collect::<Vec<String>>();

    // build the response vector
    let mut res: Vec<Item> = vec![];

    for i in 0..items.len() {
        res.push(Item {
            scraper: "FitGirl".to_string(),
            name: titles[i].to_string(),
            links: vec![items[i].to_string()],
        });
    }

    Some(res)
}

fn parse_link(link: String) -> Option<String> {
    let client = reqwest::blocking::Client::new();
    let response = match client
        .get(link)
        .header(CONTENT_TYPE, "text/html")
        .send()
        .map_err(|e| e.to_string())
    {
        Ok(r) => r,
        Err(e) => {
            log_error(&e);
            return Some(String::new());
        }
    };

    let text = match response.text().map_err(|e| e.to_string()) {
        Ok(t) => t,
        Err(e) => {
            log_error(&e);
            return Some(String::new());
        }
    };

    let document = Html::parse_document(&text);
    let magnet_selector = Selector::parse("li>a").unwrap();

    if let Some(magnet_link) = document.select(&magnet_selector).nth(1) {
        if let Some(href) = magnet_link.value().attr("href") {
            return Some(href.to_string());
        }
    }

    None
}
