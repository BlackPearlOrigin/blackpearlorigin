import { invoke } from '@tauri-apps/api/tauri';
import { log } from './Main';
import type { ScraperResponseEntry } from 'src/Typings';
import { getConfig } from './Main';

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
): Promise<ScraperResponseEntry[][] | null> => {
    if (query === '') {
        log(1, 'No query entered');
        return null;
    }

    const config = await getConfig();
    const enabledScrapers = config.enabledScrapers;

    const results = [];

    switch (true) {
        case enabledScrapers.rezi:
            const reziData = await invoke('search_rezi', {
                query: `${query}`,
            }).catch((e) => {
                log(0, `Failed to search game. Error: ${e}`);
            });
            results.push(reziData);

        case enabledScrapers.fitgirl:
            const fgData = await invoke('search_fitgirl', {
                query: `${query}`,
            }).catch((e) => {
                log(0, `Failed to search game. Error: ${e}`);
            });
            results.push(fgData);
    }

    if (results.length === 0) {
        log(1, 'No results found');
        return null;
    }

    return results as ScraperResponseEntry[][];
};
