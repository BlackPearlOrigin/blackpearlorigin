use std::{ffi, fs, path};

use rfd::FileDialog;

use crate::commands::logging::log;

use self::interface::{Game, PluginInterface};
pub mod interface;

#[derive(Debug, serde::Serialize)]
pub struct Plugin {
    pub name: String,
    pub version: String,
    pub author: String,
    pub source: String,
    pub path: path::PathBuf,
}

//  Function to get extenstin of dynamic libraries on the current platform.
//  This is used to filter out non-dynamic libraries from the plugin directory.
fn get_extension() -> ffi::OsString {
    #[cfg(target_os = "windows")]
    {
        let extension = ffi::OsString::from("dll");
        extension
    }
    #[cfg(target_os = "linux")]
    {
        let extension = ffi::OsString::from("so");
        extension
    }
    #[cfg(target_os = "macos")]
    {
        let extension = ffi::OsString::from("dylib");
        extension
    }
}

#[tauri::command]
// This function is ran everytime the user clicks the "Install plugin" button on the Preferences page
pub fn install_plugin() -> Result<(), String> {
    let extension = get_extension();
    let extension = extension.to_str().unwrap();
    let file = match FileDialog::new()
        .add_filter("Plugins", &[extension])
        .set_directory("/")
        .pick_file()
    {
        Some(file) => file.display().to_string(),
        None => return Err("No file selected".to_string()),
    };

    // Copy the plugin from the location the user selected to the plugins folder
    let file = path::Path::new(&file);
    match fs::copy(
        file,
        crate::paths::get_pbp()
            .join("plugins")
            .join(file.file_name().unwrap()),
    ) {
        Ok(_) => {}
        Err(e) => {
            log(3, &format!("Failed to copy plugin: {}", e));
            return Err(format!("Failed to copy plugin: {}", e));
        }
    }

    log(
        2,
        &format!(
            "Installed plugin with path {}",
            path::Path::new(&file).display()
        ),
    );
    Ok(())
}

#[tauri::command]
pub fn scan_plugins() -> Result<Vec<Plugin>, String> {
    let plugin_dir = crate::paths::get_pbp().join("plugins");

    let scan = fs::read_dir(plugin_dir).map_err(|e| format!("Failed to read plugin dir: {}", e));
    let scan = match scan {
        Ok(scan) => scan,
        Err(e) => {
            return Err(e);
        }
    };

    let mut plugins = vec![];
    for entry in scan {
        let entry = match entry {
            Ok(entry) => entry,
            Err(e) => {
                log(1, format!("Failed to read entry: {}", e).as_str());
                continue;
            }
        };
        //  Check if the entry is a dynamic library
        let path = entry.path();
        let entry_extension = match path.extension() {
            Some(extension) => extension,
            None => {
                log(2, "Failed to get extension");
                continue;
            }
        };
        let plugin_name = match entry.file_name().into_string() {
            Ok(plugin_name) => plugin_name,
            Err(e) => {
                log(2, format!("Failed to get plugin name: {:?}", e).as_str());
                continue;
            }
        };

        if entry_extension != get_extension() {
            log(
                3,
                format!(
                    "Skipping plugin {} because it is not a dynamic library",
                    plugin_name
                )
                .as_str(),
            );
            continue;
        }

        //  Load the plugin
        let plugin = unsafe {
            match libloading::Library::new(path.clone()) {
                Ok(plugin) => plugin,
                Err(e) => {
                    log(
                        1,
                        format!("Failed to load plugin {}: {}", plugin_name, e.to_string())
                            .as_str(),
                    );
                    continue;
                }
            }
        };

        //  Get plugin version
        let version: libloading::Symbol<fn() -> Box<dyn PluginInterface>> = unsafe {
            match plugin.get(b"version") {
                Ok(version) => version,
                Err(e) => {
                    log(
                        2,
                        format!(
                            "Failed to get version of plugin {}: {}",
                            plugin_name,
                            e.to_string()
                        )
                        .as_str(),
                    );
                    continue;
                }
            }
        };
        let version = version().version().to_string();

        //  Get plugin name
        let name: libloading::Symbol<fn() -> Box<dyn PluginInterface>> = unsafe {
            match plugin.get(b"name") {
                Ok(name) => name,
                Err(e) => {
                    log(
                        2,
                        format!(
                            "Failed to get name of plugin {}: {}",
                            plugin_name,
                            e.to_string()
                        )
                        .as_str(),
                    );
                    continue;
                }
            }
        };

        let name = name().name().to_string();

        //  Get plugin author
        let author: libloading::Symbol<fn() -> Box<dyn PluginInterface>> = unsafe {
            match plugin.get(b"author") {
                Ok(author) => author,
                Err(e) => {
                    log(
                        2,
                        format!(
                            "Failed to get author of plugin {}: {}",
                            plugin_name,
                            e.to_string()
                        )
                        .as_str(),
                    );
                    continue;
                }
            }
        };

        let author = author().author().to_string();

        //  Get plugin source
        let source: libloading::Symbol<fn() -> Box<dyn PluginInterface>> = unsafe {
            match plugin.get(b"source") {
                Ok(source) => source,
                Err(e) => {
                    log(
                        2,
                        format!(
                            "Failed to get source of plugin {}: {}",
                            plugin_name,
                            e.to_string()
                        )
                        .as_str(),
                    );
                    continue;
                }
            }
        };

        let source = source().source().to_string();

        plugins.push(Plugin {
            name,
            path,
            version,
            author,
            source,
        });
    }
    Ok(plugins)
}

#[tauri::command]
pub fn search(plugin_path: String, query: String) -> Result<Vec<Game>, String> {
    let plugin_path = path::PathBuf::from(plugin_path);
    let plugin = unsafe {
        match libloading::Library::new(plugin_path.clone()) {
            Ok(plugin) => plugin,
            Err(e) => {
                log(1, &format!("Failed to load plugin: {}", e));
                return Err(format!("Failed to load plugin: {}", e.to_string()));
            }
        }
    };

    //  Get plugin service
    let new_service: libloading::Symbol<fn() -> Box<dyn PluginInterface>> =
        unsafe { plugin.get(b"new_service") }.expect("Failed to load symbol");
    let service = new_service();

    //  Version check
    if !service.check_version("0.1.0") {
        log(1, "Plugin version mismatch");
        Err("Plugin version mismatch".to_string())
    } else {
        service.search(query)
    }
}
