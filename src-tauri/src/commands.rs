use rfd::FileDialog;
use tokio::sync::broadcast::Sender;

use crate::services::{
    self, get_list_json::get_list_json_content,
    get_list_of_music_json::get_list_of_music_json_content, music_play::MusicEvent,
};

#[tauri::command]
pub fn add_list_from_dir() -> Result<(), String> {
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
pub fn get_list_json() -> Result<String, String> {
    match get_list_json_content() {
        Ok(js) => Ok(js),
        Err(_) => Err("Get list.json Error!".to_string()),
    }
}

#[tauri::command]
pub fn get_list_of_music_json(title: String) -> Result<String, String> {
    match get_list_of_music_json_content(&title) {
        Ok(js) => Ok(js),
        Err(_) => Err(format!("Get {}.json Error!", title)),
    }
}

#[tauri::command]
pub fn handle_music_event(sender: tauri::State<Sender<MusicEvent>>, event: String) {
    let event: serde_json::Value = serde_json::from_str(&event).unwrap();
    if let Some(action) = event["action"].as_str() {
        match action {
            "play" => event["file_path"]
                .as_str()
                .map(|file_path| sender.send(MusicEvent::Play(file_path.to_owned()))),
            "pause" => Some(sender.send(MusicEvent::Pause)),
            "recovery" => Some(sender.send(MusicEvent::Recovery)),
            "volume" => event["volume"]
                .as_f64()
                .map(|volume| sender.send(MusicEvent::Volume(volume as f32))),
            _ => None,
        };
    }
}