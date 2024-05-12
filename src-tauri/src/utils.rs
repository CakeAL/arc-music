use anyhow::{anyhow, Result};
use std::path::PathBuf;
use tauri::Config;
use std::fs;

pub fn get_lists_path() -> Result<PathBuf> {
    // 获取存放歌单的目录
    let lists_path = match tauri::api::path::app_data_dir(&Config::default()) {
        Some(mut app_data_dir) => {
            app_data_dir.push(PathBuf::from(r"Arc Music/lists/"));
            // app_data_dir.push(PathBuf::from(format!("{}.json", dir_path.file_name().unwrap_or_default().to_string_lossy().into_owned())));
            app_data_dir
        }
        None => return Err(anyhow!("There is no such app data dir!")),
    };
    // 检查该目录是否存在
    if fs::read_dir(&lists_path).is_err() {
        let _ = fs::create_dir_all(&lists_path);
    }
    Ok(lists_path)
}

pub fn get_list_json_path() -> Result<PathBuf> {
    let mut list_json_path = get_lists_path()?;
    // 全部歌单数据
    list_json_path.pop();
    list_json_path.push(PathBuf::from(r"list.json"));
    Ok(list_json_path)
}