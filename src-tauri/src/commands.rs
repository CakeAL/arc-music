use rfd::FileDialog;

use crate::services::{self, get_list_json::get_list_json_content, get_list_of_music_json::get_list_of_music_json_content};

#[tauri::command]
pub async fn add_list_from_dir() -> Result<(), String> {
    if let Some(dir_path) = FileDialog::new().pick_folder() {
        match services::add_dir::add_dir(dir_path) {
            Err(_) => Err("Somewrong happened!".to_string()),
            Ok(_) => Ok(()),
        }
    } else {
        return Err("Empty dir_path!".to_string());
    }
}

#[tauri::command]
pub async fn get_list_json() -> Result<String, String> {
    match get_list_json_content() {
        Ok(js) => Ok(js),
        Err(_) => Err("Get list.json Error!".to_string()),
    }
}

#[tauri::command]
pub async fn get_list_of_music_json(title: &str) -> Result<String, String> {
    match get_list_of_music_json_content(title) {
        Ok(js) => Ok(js),
        Err(_) => Err(format!("Get {}.json Error!", title)),
    }
}
