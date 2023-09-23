// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Deserialize;
use std::collections::HashMap;
use steamlocate::SteamDir;
#[derive(Deserialize)]
struct LibraryFolders(HashMap<String, String>);

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_audiosurf_path])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn get_audiosurf_path() -> Result<String, String> {
    let mut steamdir = SteamDir::locate().unwrap();
    match steamdir.app(&4000) {
        Some(app) => Ok(app.path.to_str().unwrap().to_string().into()),
        None => Err("Audiosurf not found".into()),
    }
}
