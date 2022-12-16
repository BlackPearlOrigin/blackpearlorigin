import { ask } from "@tauri-apps/api/dialog"

export const yes = async() => await ask("are you sure", "tauri")