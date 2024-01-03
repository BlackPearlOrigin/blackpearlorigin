import { invoke } from '@tauri-apps/api/tauri';
import { log } from './Main';
import type { ScraperResponseEntry } from 'src/Typings';

/**
 * Typescript Function -> Rust Function
 * - Runs the search function passing the
 *   same arguments from the TS function to the Rust
 *   function
 *
 * @param {string} query
 * @returns {Promise<ScraperResponseEntry[]>} Array of SearchedGame
 */
export const searchGame = async (
    query: string
): Promise<ScraperResponseEntry[] | null> => {
    if (query === '') {
        log(1, 'No query entered');
        return null;
    }

    const data = await invoke('search_rezi', {
        query: `${query}`,
    }).catch((e) => {
        log(0, `Failed to search game. Error: ${e}`);
    });

    return data as ScraperResponseEntry[];
};
