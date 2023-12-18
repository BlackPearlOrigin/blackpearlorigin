use reqwest::header::AUTHORIZATION;
use crate::commands::scrapers::Item;

#[derive(serde::Serialize)]
struct Payload {
    q: String,
    limit: i32
}

#[derive(serde::Deserialize)]
struct Response {
    hits: Vec<Hit>,
}

#[derive(serde::Deserialize)]
struct Hit {
    id: String,
    link: String,
    title: String,
    igdb_url: String,
    site: String
}

pub async fn rezi_scraper(query: String) -> Vec<Item> {
    let payload = Payload {
        q: query,
        limit: 15
    };
    let payload_ser = serde_json::to_string(&payload).unwrap();

    let result = reqwest::Client::new()
        .get("https://search.rezi.one/indexes/rezi/search")
        .header(AUTHORIZATION, "e2a1974678b37386fef69bb3638a1fb36263b78a8be244c04795ada0fa250d3d")
        .body(payload_ser)
        .send()
        .await
        .expect("failed to make request")
        .text()
        .await;

    let result_json: Response = serde_json::from_str(&result.unwrap()).unwrap();
    let mut to_frontend: Vec<Item> = vec![];

    for hit in result_json.hits {    
        let links = vec![hit.link];

        let res = Item {
            scraper: "Rezi".to_string(),
            name: hit.title,
            links 
        };
        
        to_frontend.push(res);
    }

    return to_frontend;
}