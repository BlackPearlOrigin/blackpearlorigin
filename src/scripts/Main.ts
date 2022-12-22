import { readTextFile, BaseDirectory } from '@tauri-apps/api/fs';
import { locale } from '../locale/i18n';

// TS Function
// - Gets the current locale from config.json
export async function getCurrentLocale() {
	const config: string = await readTextFile('config.json', {
		dir: BaseDirectory.AppLocalData,
	});

	let configParsed = JSON.parse(config);
	return configParsed.currentLang as string;
}

// TS Function
// - Loads the current locale
export async function loadLocale() {
	locale.set('en');
	//locale.set(await getCurrentLocale());
}
