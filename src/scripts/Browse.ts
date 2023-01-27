import { BaseDirectory, join } from '@tauri-apps/api/path';
import { readTextFile, removeFile } from '@tauri-apps/api/fs';
import { invoke } from '@tauri-apps/api/tauri';
import type { TempScrapers, SearchResults } from './Interfaces';

/**
 * Typescript Function
 * - Reads the values from temp/scrapers.json and returns it
 *
 * @returns {Promise<TempScrapers>} Scrapers read from temp/scrapers.json
 */
export const getScrapers = async (): Promise<TempScrapers> => {
	// Read the file from AppData path
	const file: string = await readTextFile('temp/scrapers.json', {
		dir: BaseDirectory.AppLocalData,
	}).catch((e) => {
		invoke('log', {
			logLevel: 0,
			logMessage: 'Failed to open file',
		});

		return e;
	});

	// Parse the JSON and return it
	const json = JSON.parse(file);
	return json;
};

/**
 * Typescript Function -> Rust Function
 * - Runs the handle_scraper function passing the
 *   same arguments from the TS function to the Rust
 *   function
 *
 * @param {string} path
 * @param {string} query
 * @returns {Promise<void>} Nothing
 */
export const searchGame = async (
	path: string,
	query: string
): Promise<void> => {
	// Params:
	// title: Game title
	// path: Path to the scraper

	// Invoke the rust backend for initializing the scraper when a user presses the search button
	if (path.endsWith('.exe')) {
		await invoke('handle_scraper', { path: path, query: query });
	}
};

/**
 * Typescript Function
 * - Returns the JSON read from queries/results.json as an Object
 *
 * @returns {Promise<SearchResults>} Search results gotten from queries/results.json
 */
export const displayResults = async (): Promise<SearchResults> => {
	let locationCache: string = await join('queries', 'results.json');

	// Reads the cache file
	const file = await readTextFile(locationCache, {
		dir: BaseDirectory.AppLocalData,
	}).catch((e) => {
		invoke('log', {
			logLevel: 0,
			logMessage: 'Failed to open file',
		});

		return e;
	});

	// Parses that same file and then returns it
	const json = JSON.parse(file);

	await removeResults();
	return json;
};

/**
 * Typescript function
 * - Removes the queries/results.json file
 *
 * @returns {Promise<void>} Nothing
 */
export const removeResults = async (): Promise<void> => {
	await removeFile('queries/results.json', {
		dir: BaseDirectory.AppLocalData,
	}).catch((e) => {
		invoke('log', {
			logLevel: 0,
			logMessage: 'Failed to remove file',
		});

		return e;
	});
};

/**
 * Typescript Function
 * - Handles the keypress, if the keypress is "Enter",
 *   searches for a game
 *
 * @param {string} pressedKey
 * @param {string} scraperPath
 * @param {string} searchQuery
 * @returns {Promise<SearchResults>} Search results gathered by {@link searchGame}
 */
export const handleKeypress = (
	pressedKey: string,
	scraperPath: string,
	searchQuery: string
): Promise<SearchResults> => {
	let key = pressedKey;

	if (key.toString() == 'Enter') {
		const searchResults = searchGame(scraperPath, searchQuery).then(() => {
			return displayResults();
		});

		console.log(searchResults);
		return searchResults;
	}
};
