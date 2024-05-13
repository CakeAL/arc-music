use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct List {
    pub title: String,
    pub musics_num: usize,
    pub create_at: i64,
    pub folder_path: String,
    pub picture_base64: String,
}