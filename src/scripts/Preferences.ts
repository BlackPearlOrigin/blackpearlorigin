import { invoke } from '@tauri-apps/api/tauri';
import { ask, message } from '@tauri-apps/api/dialog';
import { BaseDirectory, writeTextFile } from '@tauri-apps/api/fs';

// TS Function -> Rust Function
// Invokes a function that installs a scraper
export function installScraper() {
	invoke('install_scraper');
}

// TS Function -> Rust Function
// Invokes a function that wipes the library
export async function wipeLibrary() {
	const areYouSure = await ask(
		"Are you sure, this action can't be undone",
		'Library Deletion'
	);
	if (areYouSure) {
		invoke('wipe_library');
		await message('Library successfully deleted', 'Library Deletion');
	}
}

export async function saveData(lang: string) {
	let dataObj = {
		currentLang: lang,
	};

	let dataObjString = JSON.stringify(dataObj);

	await writeTextFile('config.json', dataObjString, {
		dir: BaseDirectory.AppLocalData,
	});
}
