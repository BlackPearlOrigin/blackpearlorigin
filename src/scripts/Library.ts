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
// - Edits the game data, TBD
export function editGame(name: string) {
	invoke('edit_from_db', { name: name });
}

// TS Function -> Rust Function
// - Gets all games from the db
export async function getGames() {
	const games = await invoke('get_from_db');
	return games;
}
