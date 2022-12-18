import { invoke } from '@tauri-apps/api/tauri';

export function downloadTest() {
    invoke('download_test');
}