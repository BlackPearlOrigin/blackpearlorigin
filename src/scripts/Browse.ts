import { invoke } from '@tauri-apps/api/tauri';
import type { Plugin } from './Interfaces';
import { log } from './Main';
/**
 * Typescript Function -> Rust Function
 * - Runs the scan_plugins function from the Rust backend
 *  and returns the data
 *
 * @returns {Promise<Plugin[]>} Array of plugins
 */

export const getPlugins = async (): Promise<Plugin[]> => {
    const data: Plugin[] = await invoke('scan_plugins')
        .then((data: Plugin[]) => {
            return data;
        })
        .catch((e: string) => {
            log(0, `Failed to scan plugins. Error: ${e}`);
            return [];
        });
    
    return data;
};

/**
 * Typescript Function -> Rust Function
 * - Runs the search function passing the
 *   same arguments from the TS function to the Rust
 *   function
 *
 * @param {string} pluginPath
 * @param {string} query
 * @returns {Promise<string>} Array of SearchedGame
 */
export const searchGame = async (
    pluginPath: string,
    query: string
): Promise<string> => {
    // Params:
    // - pluginPath: string
    // - query: string
    if (query === '') {
        log(1, 'No query entered');
        return "{}";
    }

    if (pluginPath === '') {
        log(1, 'No plugin selected!');
        return "{}";
    
    }
    const data = await invoke('search', {
        luaFile: pluginPath,
        query: query,
    }).catch((e: string) => {
            log(0, `Failed to search game. Error: ${e}`);
            return {};
    });
  
    return data as string
};
/**
 * Typescript Function
 * - Handles the keypress, if the keypress is "Enter",
 *   searches for a game
 *
 * @param {string} pluginPath
 * @param {string} search
 * @returns {Promise<string>} Search results gathered by {@link searchGame}
 */
export const handleKeypress = async (
    pluginPath: string,
    search: string
): Promise<string> => {
    const searchResults = await searchGame(pluginPath, search)
    return searchResults as string;
};
