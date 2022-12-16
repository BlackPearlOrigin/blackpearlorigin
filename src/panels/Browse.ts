import { BaseDirectory } from "@tauri-apps/api/path";
import { readTextFile } from "@tauri-apps/api/fs";

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
}