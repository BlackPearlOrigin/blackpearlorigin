use crate::{commands::logging::log, paths::get_bpo};
use mlua::Lua;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf, process, vec};

#[derive(Serialize, Deserialize)]
pub struct Plugin {
    name: String,
    description: String,
    author: String,
    source: String,
    location: String,
}

#[derive(Serialize, Deserialize)]
pub struct Metadata {
    name: String,
    description: String,
    author: String,
    source: String,
}

#[derive(Serialize, Deserialize)]
pub struct Link {
    label: String,
    url: String,
}

#[derive(Serialize, Deserialize)]
pub struct Game {
    name: String,
    links: Vec<Link>,
}

#[tauri::command]
pub fn scan_plugins() -> Result<Vec<Plugin>, String> {
    let folders_rd = get_bpo()
        .join("plugins")
        .read_dir()
        .map_err(|e| e.to_string())?;

    let mut plugins = vec![];

    for entry in folders_rd.flatten() {
        if !entry.path().is_dir() {
            continue;
        }

        let plugin_path = entry.path().join("plugin.lua");
        let plugin_info_path = entry.path().join("plugin_info.json");

        println!("{}", plugin_path.display());
        if !plugin_path.exists() {
            log(
                1,
                format!(
                    "Plugin file is missing for plugin {}",
                    entry.file_name().to_string_lossy()
                ),
            );
            continue;
        }

        if !plugin_info_path.exists() {
            log(
                1,
                format!(
                    "Plugin info file is missing for plugin {}",
                    entry.file_name().to_string_lossy()
                ),
            );
            continue;
        }

        if let Ok(content) = fs::read_to_string(plugin_info_path) {
            if let Ok(deserialized) = serde_json::from_str::<Metadata>(&content) {
                let plugin = Plugin {
                    name: deserialized.name,
                    description: deserialized.description,
                    author: deserialized.author,
                    source: deserialized.source,
                    location: plugin_path.to_string_lossy().to_string(),
                };

                plugins.push(plugin);
            } else {
                log(
                    1,
                    format!(
                        "Plugin info file format for plugin {} is invalid",
                        entry.file_name().to_string_lossy()
                    ),
                );
                continue;
            }
        } else {
            log(
                1,
                format!(
                    "Plugin info file for plugin {} could not be read",
                    entry.file_name().to_string_lossy()
                ),
            );
            continue;
        }
    }

    Ok(plugins)
}

#[tauri::command]
pub fn uninstall_plugin(plugin: Plugin) -> Result<(), String> {
    let plugin_location = PathBuf::from(plugin.location);
    if let Some(f) = plugin_location.parent() {
        fs::remove_dir(f).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
pub fn search(lua_file: String, query: String) -> Result<Vec<Game>, String> {
    let lua = Lua::new();

    let lua_path = &PathBuf::from(lua_file);
    let folder = match lua_path.parent() {
        Some(p) => p,
        None => {
            log(
                0,
                "plugin file path doesn't have a parent (how did you mess that up???)".to_string(),
            );
            process::exit(0)
        }
    };

    let results_path = folder.join("results");
    if !results_path.exists() {
        fs::create_dir(results_path).map_err(|e| e.to_string())?;
    }

    lua.globals()
        .set("query", query)
        .map_err(|e| e.to_string())?;
    
    let code = fs::read_to_string(lua_path).map_err(|e| e.to_string())?;
    lua.load(&code).exec().map_err(|e| e.to_string())?;

    let contents = fs::read_to_string(folder.join("result.json")).map_err(|e| e.to_string())?;
    let games: Vec<Game> = serde_json::from_str(&contents).map_err(|e| e.to_string())?;

    Ok(games)
}
