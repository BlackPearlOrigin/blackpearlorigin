import { invoke } from '@tauri-apps/api/tauri';

export async function runGame(path: string) {
	invoke('run_game', { path: path });
}

export async function deleteGame(name: string) {
	invoke('delete_from_db', { name: name });
}

export function editGame(name: string) {
	invoke('edit_from_db', { name: name });
}

export async function getGames() {
	const games = await invoke('get_from_db');
	return games;
}
