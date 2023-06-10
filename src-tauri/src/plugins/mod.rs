use std::{ffi, fs, path};

use rfd::FileDialog;

use crate::commands::logging::log;

use self::interface::{Game, PluginInterface};
pub mod interface;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Plugin {
    pub name: String,
    pub version: String,
    pub author: String,
    pub source: String,
    pub description: String,
    pub path: path::PathBuf,
}

//  Function to get extenstin of dynamic libraries on the current platform.
//  This is used to filter out non-dynamic libraries from the plugin directory.
fn get_extension() -> ffi::OsString {
    #[cfg(target_os = "windows")]
    {
        ffi::OsString::from("dll")
    }
    #[cfg(target_os = "linux")]
    {
        ffi::OsString::from("so")
    }
    #[cfg(target_os = "macos")]
    {
        ffi::OsString::from("dylib")
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
        crate::paths::get_bpo()
            .join("plugins")
            .join(file.file_name().unwrap()),
    ) {
        Ok(_) => {}
        Err(e) => {
            log(2, format!("Failed to copy plugin: {}", e));
            return Err(format!("Failed to copy plugin: {}", e));
        }
    }

    log(2, format!("Installed plugin with path {}", file.display()));
    Ok(())
}

#[tauri::command]
pub fn uninstall_plugin(plugin: Plugin) -> Result<(), String> {
    let plugin_path = plugin.path;
    match fs::remove_file(plugin_path) {
        Ok(_) => {}
        Err(e) => {
            log(2, format!("Failed to remove plugin: {}", e));
            return Err(format!("Failed to remove plugin: {}", e));
        }
    }
    Ok(())
}

#[tauri::command]
pub fn scan_plugins() -> Result<Vec<Plugin>, String> {
    let plugin_dir = crate::paths::get_bpo().join("plugins");
    let mut plugins = vec![];

    let rd = match fs::read_dir(plugin_dir) {
        Ok(rd) => rd,
        Err(e) => {
            log(0, "Error reading plugin directory".to_owned());
            return Err(e.to_string());
        }
    };

    for entry in rd.flatten() {
        let entry_path = entry.path();
        let plugin_name = entry.file_name();

        //  Check if the entry is a dynamic library
        if let Some(ext) = entry_path.extension() {
            if ext != get_extension() {
                continue;
            }
        } else {
            continue;
        }

        //  Load the plugin
        let plugin = match unsafe { libloading::Library::new(&entry_path) } {
            Ok(plugin) => plugin,
            Err(e) => {
                log(
                    0,
                    format!(
                        "Failed to load plugin {}: {}",
                        plugin_name.to_string_lossy(),
                        e
                    ),
                );
                continue;
            }
        };

        let new_service: libloading::Symbol<fn() -> Box<dyn PluginInterface>> =
            unsafe { plugin.get(b"new_service").expect("Failed to load symbol") };
        let service = new_service();

        let plugin = Plugin {
            name: service.name().to_owned(),
            version: service.version().to_owned(),
            author: service.author().to_owned(),
            source: service.source().to_owned(),
            description: service.description().to_owned(),
            path: entry_path,
        };

        plugins.push(plugin);
    }

    Ok(plugins)
}

#[tauri::command]
pub fn search(plugin_path: String, query: String) -> Result<Vec<Game>, String> {
    let plugin_path = path::PathBuf::from(plugin_path);
    let plugin = match unsafe { libloading::Library::new(plugin_path) } {
        Ok(plugin) => plugin,
        Err(e) => {
            log(0, format!("Failed to load plugin: {}", e));
            return Err(format!("Failed to load plugin: {}", e));
        }
    };

    //  Get plugin service
    let new_service: libloading::Symbol<fn() -> Box<dyn PluginInterface>> =
        unsafe { plugin.get(b"new_service") }.expect("Failed to load symbol");
    let service = new_service();

    //  Version check
    if !service.check_version("0.1.0") {
        log(0, "Plugin version mismatch".to_owned());
        Err("Plugin version mismatch".to_string())
    } else {
        service.search(query)
    }
}
