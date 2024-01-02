use crate::commands::logging::log_error;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};

#[derive(serde::Serialize, Debug)]
pub struct Item {
    pub scraper: String,
    pub name: String,
    pub links: Vec<String>,
}

#[derive(serde::Serialize)]
struct Payload {
    q: String,
    limit: i32,
}

#[derive(serde::Deserialize)]
struct Response {
    hits: Vec<Hit>,
}

#[derive(serde::Deserialize)]
struct Hit {
    link: String,
    title: String,
}

#[tauri::command]
pub fn search_rezi(query: &str) -> Option<Vec<Item>> {
    let payload = Payload {
        q: query.to_owned(),
        limit: 16,
    };

    let payload_str = match serde_json::to_string(&payload).map_err(|e| e.to_string()) {
        Ok(str) => str,
        Err(e) => {
            log_error(&e);
            return None;
        }
    };

    let client = reqwest::blocking::Client::new();

    let response = match client
        .post("https://search.rezi.one/indexes/rezi/search")
        .header(
            AUTHORIZATION,
            "Bearer e2a1974678b37386fef69bb3638a1fb36263b78a8be244c04795ada0fa250d3d",
        )
        .header(CONTENT_TYPE, "application/json")
        .body(payload_str)
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

    let result_json: Response = match serde_json::from_str(&text).map_err(|e| e.to_string()) {
        Ok(j) => j,
        Err(e) => {
            log_error(&e);
            return None;
        }
    };

    let mut items: Vec<Item> = vec![];

    for hit in result_json.hits {
        let links = vec![hit.link];

        let res = Item {
            scraper: "Rezi".to_string(),
            name: hit.title,
            links,
        };

        items.push(res);
    }

    Some(items)
}
