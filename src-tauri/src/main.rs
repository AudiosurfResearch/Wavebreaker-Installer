// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Deserialize;
use std::{collections::HashMap, io::Cursor, path};
use steamlocate::{SteamApp, SteamDir};
#[derive(Deserialize)]
struct LibraryFolders(HashMap<String, String>);

trait ResultExt<T> {
    fn map_to_tauri(self) -> Result<T, String>;
}

impl<T, E: ToString> ResultExt<T> for Result<T, E> {
    fn map_to_tauri(self) -> Result<T, String> {
        self.map_err(|err| err.to_string())
    }
}

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
async fn get_audiosurf_path() -> Result<String, String> {
    println!("Trying to find Steam...");
    let mut steamdir = SteamDir::locate().unwrap();
    println!("Steam at {:?}", steamdir.path);
    let apps: &HashMap<u32, Option<SteamApp>> = steamdir.apps();
    for (_, app) in apps {
        if let Some(app) = app {
            if app.app_id == 12900 {
                println!("Audiosurf found at {:?}", app.path);
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
fn is_valid_audiosurf_folder(path: &str) -> bool {
    return path::Path::new(path).join("engine\\channels").exists();
}

#[tauri::command]
async fn install(path: String) -> Result<(), String> {
    if !is_valid_audiosurf_folder(&path) {
        return Err("Invalid Audiosurf folder!".into());
    }

    let target = "https://github.com/AudiosurfResearch/Wavebreaker-Hook/releases/latest/download/Wavebreaker-Package.zip";
    let response = reqwest::get(target).await.map_to_tauri()?;
    let content = response.bytes().await.map_to_tauri()?;

    println!("Checking for old files");
    let old_files = vec![
        "engine\\channels\\Wavebreaker-Hook.dll",
        "engine\\channels\\wavebreakerclient.dll",
        "engine\\SongSelector\\RadioBrowser.cgr",
        "engine\\Wavebreaker-Hook.ini",
        "engine\\Wavebreaker-Client.toml",
    ];
    for file in old_files {
        let file_path = path::Path::new(&path).join(file);
        if file_path.exists() {
            println!("Removing {}", file);
            std::fs::remove_file(file_path).map_to_tauri()?;
        }
    }

    let target_dir = path::Path::new(&path).join("engine");
    zip_extract::extract(Cursor::new(content.to_vec()), &target_dir, true).map_to_tauri()?;

    return Ok(());
}
