// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Deserialize;
use std::{collections::HashMap, fs::File, io::copy, path};
use steamlocate::{SteamApp, SteamDir};
use tempfile::Builder;
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
fn is_valid_audiosurf_folder(path: &str) -> bool {
    return path::Path::new(path).join("engine\\channels").exists();
}

#[tauri::command]
async fn install(path: String) -> Result<(), String> {
    if !is_valid_audiosurf_folder(&path) {
        return Err("Invalid Audiosurf folder!".into());
    }

    let tmp_dir = Builder::new()
        .prefix("wavebreaker-installer")
        .tempdir()
        .map_err(|err| err.to_string());
    let target = "https://github.com/AudiosurfResearch/Wavebreaker-Hook/releases/latest/download/Wavebreaker-Package.zip";
    let response = reqwest::get(target).await.map_err(|err| err.to_string())?;

    let mut dest = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        println!("file to download: '{}'", fname);
        let fname = tmp_dir.unwrap().path().join(fname);
        println!("will be located under: '{:?}'", fname);
        File::create(fname).map_err(|err| err.to_string())?
    };
    let content = response.text().await.map_err(|err| err.to_string())?;
    copy(&mut content.as_bytes(), &mut dest).map_err(|err| err.to_string())?;

    let target_dir = path::Path::new(&path);
    zip_extract::extract(&dest, &target_dir, true).map_err(|err| err.to_string())?;

    return Ok(());
}
