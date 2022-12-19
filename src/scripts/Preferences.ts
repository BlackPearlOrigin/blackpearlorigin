import { invoke } from '@tauri-apps/api/tauri';

export function installScraper() {
    invoke('install_scraper');
}

export function wipeLibrary() {
    invoke('wipe_library');
}