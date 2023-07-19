import { invoke } from '@tauri-apps/api/tauri';
import { ask, message } from '@tauri-apps/api/dialog';
import { BaseDirectory, writeTextFile } from '@tauri-apps/api/fs';
import type { Plugin } from './Interfaces';
import exp from 'constants';

/**
 * Typescript Function -> Rust Function
 * - Invokes a function that installs a plugin
 *
 * @returns Nothing
 */
export const installPlugin = async () => await invoke('install_plugin');

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

export const steamLogin = async () => {
    await invoke('steam_login');
};

export const steamUnlink = () => {
    localStorage.removeItem('steamData');
};

let socket = new WebSocket('ws://localhost:5274/ws');
console.log('Attempting Connection...');
socket.onopen = () => {
    console.log('Successfully Connected');
};
socket.onclose = (event) => {
    console.log('Socket Closed Connection: ', event);
};

socket.onmessage = (msg) => {
    if (msg.data.includes('steam')) {
        const data = JSON.parse(msg.data);
        localStorage.setItem(
            'steamData',
            JSON.stringify(data.response.players[0])
        );
    }
};
