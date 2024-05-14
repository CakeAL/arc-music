use std::{fs::File, io::Read};

use anyhow::Result;

use crate::utils;

pub fn get_list_of_music_json_content(title: &String) -> Result<String> {
    let mut list_of_music_json_path = utils::get_lists_path()?;
    list_of_music_json_path.push(title);
    // println!("{:?}", list_of_music_json_path);
    let mut file = File::open(&list_of_music_json_path)?;
    let mut list_of_music_string = String::new();
    file.read_to_string(&mut list_of_music_string)?;
    Ok(list_of_music_string)
}