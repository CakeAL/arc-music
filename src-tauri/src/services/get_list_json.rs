use std::{fs::File, io::Read};

use anyhow::Result;

use crate::utils;

pub fn get_list_json_content() -> Result<String> {
    let list_json_path = utils::get_list_json_path()?;
    let mut file = File::open(&list_json_path)?;
    let mut music_list_string = String::new();
    file.read_to_string(&mut music_list_string)?;
    Ok(music_list_string)
}
