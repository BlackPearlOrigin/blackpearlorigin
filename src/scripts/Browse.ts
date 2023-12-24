import { invoke } from '@tauri-apps/api/tauri';
import { log } from './Main';

/**
 * Typescript Function -> Rust Function
 * - Runs the scan_plugins function from the Rust backend
 *  and returns the data
 *
 * @returns {Promise<Plugin[]>} Array of plugins
 */

/**
 * Typescript Function -> Rust Function
 * - Runs the search function passing the
 *   same arguments from the TS function to the Rust
 *   function
 *
 * @param {string} query
 * @returns {Promise<string>} Array of SearchedGame
 */
export const searchGame = async (query: string): Promise<string> => {
    if (query === '') {
        log(1, 'No query entered');
        return '{}';
    }

    const data = await invoke('run_scraper', {
        query: query,
        scrapers: ['rezi'],
    }).catch((e: string) => {
        log(0, `Failed to search game. Error: ${e}`);
        return {};
    });

    return data as string;
};
