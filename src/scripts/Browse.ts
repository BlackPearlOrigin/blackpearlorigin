import { BaseDirectory } from '@tauri-apps/api/path';
import { readTextFile } from '@tauri-apps/api/fs';
import { invoke } from '@tauri-apps/api/tauri';

// TS Function
// - Reads the values from scrapers.json and then returns it
export async function browse() {
	// Read the file from AppData path
	const file = await readTextFile('temp/scrapers.json', {
		dir: BaseDirectory.AppLocalData,
	});

	// Parse the JSON and return it
	const json = JSON.parse(file);
	return json;
}

// Exported TS Function -> Rust Function
// - Invoke a function that runs the scraper in path argument
export async function search(path: string, query: string) {
	// Params:
	// title: Game title
	// path: Path to the scraper

	// Invoke the rust backend for initializing the scraper when a user presses the search button
	if (path.endsWith('.exe')) {
		invoke('handle_scraper', { path: path, query: query });
	}
}

// Exported TS Function -> Rust Function
// - Returns the results of queries/cache.json
export async function displayResults() {
	// Reads the cache file
	const file = await readTextFile('queries/results.json', {
		dir: BaseDirectory.AppLocalData,
	});

	// Parses that same file and then returns it
	const json = JSON.parse(file);
	return json;
}
