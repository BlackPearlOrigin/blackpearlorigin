import { readTextFile, BaseDirectory } from '@tauri-apps/api/fs';
import { invoke } from '@tauri-apps/api/tauri';
import { locale } from '../locale/i18n';

// TS Function
// - Gets the current locale from config.json
export async function getCurrentLocale() {
	const config: string = await readTextFile('config.json', {
		dir: BaseDirectory.AppLocalData,
	}).catch(() => {
		invoke('log', {
			logLevel: 0,
			logMessage: 'Failed to read file',
		});

		return '';
	});

	let configParsed = JSON.parse(config);
	return configParsed.currentLang as string;
}

// TS Function
// - Loads the current locale
export async function loadLocale() {
	locale.set(await getCurrentLocale());
}
