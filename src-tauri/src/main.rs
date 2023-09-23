// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Deserialize;
use std::{collections::HashMap, path};
use steamlocate::{SteamApp, SteamDir};
#[derive(Deserialize)]
struct LibraryFolders(HashMap<String, String>);

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_audiosurf_path,
            is_valid_audiosurf_folder,
            install
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_audiosurf_path() -> Result<String, String> {
    let mut steamdir = SteamDir::locate().unwrap();
    let apps: &HashMap<u32, Option<SteamApp>> = steamdir.apps();
    for (_, app) in apps {
        if let Some(app) = app {
            if app.app_id == 12900 {
                return Ok(app.path.to_str().unwrap().to_string());
            }
        }
    }
    Err(
        "Couldn't locate Audiosurf! Make sure you own the game on Steam and have it installed!"
            .into(),
    )
}

#[tauri::command]
fn is_valid_audiosurf_folder(path: String) -> bool {
    return path::Path::new(&path).join("engine\\channels").exists();
}

#[tauri::command]
fn install(path: String) -> Result<(), String> {
    if !is_valid_audiosurf_folder(path) {
        return Err("Invalid Audiosurf folder!".into());
    }
    return Ok(());
}
