// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use arc_music::services::music_play::MusicService;

#[tokio::main]
async fn main() {
    let music_services = MusicService::new();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            arc_music::commands::add_list_from_dir,
            arc_music::commands::get_list_json,
            arc_music::commands::get_list_of_music_json,
            arc_music::commands::handle_music_event,
        ])
        .manage(music_services.event_sender)
        .manage(music_services.sink)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
