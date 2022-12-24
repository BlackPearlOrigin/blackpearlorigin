import { invoke } from '@tauri-apps/api/tauri';

// TS Function -> Rust Function
// - Starts the game defined in the path arg
export async function runGame(path: string) {
	invoke('run_game', { path: path });
}

// TS Function -> Rust Function
// - Deletes the game from the db
export async function deleteGame(id: number) {
	invoke('delete_from_db', { id: id });
}

// TS Function -> Rust Function
// - Gets all games from the db
export async function getGames() {
	const games = await invoke('get_from_db');
	return games;
}

// TS Function -> Rust Function
// - Saves game to db
export function saveData(title: string, executablePath: string, description: string, image_path: string) {
	invoke('save_to_db', {
		title: title,
		exePath: executablePath,
		description: description,
		image: image_path,
	});
}

// TS Function -> Rust Function
// - Edits game in db
export function editData(id: number, title: string, executablePath: string, description: string, image_path: string) {
	invoke('edit_in_db', {
		id: id,
		name: title,
		executable: executablePath,
		description: description,
		image: image_path,
	});
}