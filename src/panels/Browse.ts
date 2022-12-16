import { BaseDirectory } from "@tauri-apps/api/path";
import { readTextFile } from "@tauri-apps/api/fs";
import { invoke } from '@tauri-apps/api/tauri';

export async function browse() {
    // Read the file from AppData path
    const file = await readTextFile('temp/scrapers.json', {
        dir: BaseDirectory.AppLocalData
    })

    // Parse the JSON and return it
    const json = JSON.parse(file)
    return json
}

export async function search(path: string, query: string) {
    // Params:
    // title: Game title
    // path: Path to the scraper

    // Invoke the rust backend for initializing the scraper when a user presses the search button
    if (path.endsWith(".exe")) {
        invoke('handle_scraper', { scraper: 0, path: path, query: query })
    }
}

export async function displayResults() {
    const file = await readTextFile('queries/cache.json', {
        dir: BaseDirectory.AppLocalData
    })

    const json = JSON.parse(file)
    return json
}