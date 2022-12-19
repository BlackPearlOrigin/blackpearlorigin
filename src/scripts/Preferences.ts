import { invoke } from '@tauri-apps/api/tauri';
import { ask, message } from '@tauri-apps/api/dialog';

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
