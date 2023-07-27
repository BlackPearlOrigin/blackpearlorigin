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
export const installPlugin = async (): Promise<number> => {
    let returnValue = await invoke('file_dialog');
    return returnValue as number;
};
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
    updaterToggle: boolean,
    cssUrl: string
): Promise<void> => {
    let dataObj = {
        currentLang: lang,
        updater: updaterToggle,
        cssUrl: cssUrl,
    };

    switchTheme(cssUrl);

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
 * @returns nothing
 */
export const uninstallPlugin = async (plugin: Plugin) => {
    await invoke('uninstall_plugin', { plugin: plugin });
};

/*
 * Typescript Function
 * - Theme switcher poggers
 */
export const switchTheme = (cssUrl: string) => {
    let head = document.getElementsByTagName('head')[0];
    // let link = document.createElement('link');

    let link = getOrCreateElement('custom-stylesheet');

    link.setAttribute('href', cssUrl);
    link.setAttribute('type', 'text/css');
    link.setAttribute('rel', 'stylesheet');

    head.appendChild(link);
};

const getOrCreateElement = (id: string) => {
    // Check if the element already exists in the DOM
    const element = document.getElementById(id);
    if (element) {
        // The element exists, return a reference to it
        return element;
    } else {
        // The element doesn't exist, create a new one
        const newElement = document.createElement('link');
        newElement.setAttribute('id', id);
        return newElement;
    }
};
