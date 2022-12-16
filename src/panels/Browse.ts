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

export async function search(title: string, path: string) {
    // Invoke the rust backend for initializing the scraper when a user presses the search button
    if (path.endsWith(".exe")) {
        invoke('handle_scraper', { scraper: 0 })
    } else if (path.endsWith(".py")) {
        invoke('handle_scraper', { scraper: 1 })
    }
}