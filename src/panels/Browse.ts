import { BaseDirectory } from "@tauri-apps/api/path";
import { readTextFile } from "@tauri-apps/api/fs";
import { invoke } from '@tauri-apps/api/tauri';

export async function browse() {
    let pbpPath: string;

    console.log(pbpPath);
    const file = await readTextFile('temp/scrapers.json', {
        dir: BaseDirectory.AppLocalData
    })

    const json = JSON.parse(file)

    return json
}

export async function search(title: string, path: string) {
    // To be done...

    // Invoke the rust backend for initializing the scraper when a user presses the search button
    if (title.endsWith(".exe")) {
        invoke('handle_scraper', { number: 0 })
    } else if (title.endsWith(".py")) {
        invoke('handle_scraper', { number: 1 })
    }
}