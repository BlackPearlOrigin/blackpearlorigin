import { invoke } from '@tauri-apps/api/tauri';
import { ask, message } from '@tauri-apps/api/dialog';
import { BaseDirectory, writeTextFile } from '@tauri-apps/api/fs';
import type { Plugin } from './Interfaces';

/**
 * Typescript Function -> Rust Function
 * - Invokes a function that installs a plugin
 *
 * @returns Nothing
 */
export const installPlugin = async () => await invoke('file_dialog').then((file) => {
   console.log(file) 
});

/**
 * Typescript Function -> Rust Function
 * - Opens a pop-up window, then if the user selects yes,
 *   wipes the library
 *
 * @returns {Promise<void>} Nothing
 */
export const wipeLibrary = async (): Promise<void> => {
    const areYouSure = await ask(
        'Are you sure? This action can not be undone.',
        'Library Deletion'
    );
    if (areYouSure) {
        invoke('wipe_library');
        await message('Library successfully deleted', 'Library Deletion');
    }
};

/**
 * Typescript Function
 * - Saves the selected language to config.json
 *
 * @param {string} lang
 */
export const saveData = async (
    lang: string,
    updaterToggle: boolean
): Promise<void> => {
    let dataObj = {
        currentLang: lang,
        updater: updaterToggle,
    };

    let dataObjString = JSON.stringify(dataObj);

    await writeTextFile('config.json', dataObjString, {
        dir: BaseDirectory.AppLocalData,
    }).catch(() => {
        invoke('log', {
            logLevel: 0,
            logMessage: 'Failed to write file',
        });

        return '';
    });
};

/**
 * Typescript Function -> Rust Function
 * - Invokes a function that uninstalls a plugin
 *
 * @returns Nothing
 */
export const uninstallPlugin = async (plugin: Plugin) => {
    await invoke('uninstall_plugin', { plugin: plugin });
};
