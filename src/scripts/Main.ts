import { readTextFile, BaseDirectory } from '@tauri-apps/api/fs';
import { invoke } from '@tauri-apps/api/tauri';
import { locale } from '../locale/i18n';
import { switchTheme } from './Preferences';

// TS Function
// - Gets the current locale from config.json
export async function getConfig() {
    const config: string = await readTextFile('config.json', {
        dir: BaseDirectory.AppLocalData,
    }).catch(() => {
        log(0, 'Failed to read config.json');

        return '';
    });

    let configParsed = JSON.parse(config);
    return configParsed;
}

// TS Function
// - Loads the current locale
export async function loadLocale() {
    const config = await getConfig();
    locale.set(config.currentLang);
    switchTheme(config.cssUrl)
}

// TS Function -> Rust Function
// - Logs a message to the Rust backend
export function log(logLevel: number, logMessage: string) {
    const levels: string[] = ['ERROR', 'WARNING', 'INFO'];
    console.log(`[${levels[logLevel]}]: ${logMessage}`);
    invoke('log', {
        logLevel: logLevel,
        logMessage: `From TS: ${logMessage}`,
    });
}

// Defines a function that checks if the same string is empty
export const isEmpty = (string: string) => {
    return string === undefined || string.length === 0 || !string.trim();
};
