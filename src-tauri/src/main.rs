// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            arc_music::commands::add_list_from_dir,
            arc_music::commands::get_list_json,
            arc_music::commands::get_list_of_music_json,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
