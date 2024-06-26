use anyhow::{anyhow, Result};
use chrono::Local;
use id3::{Tag, TagLike};
use std::fs::{File, self};
use std::io::{Read, Write};
use std::path::PathBuf;
use base64::prelude::*;


use crate::models::list::List;
use crate::models::music::Music;
use crate::utils;

pub fn add_dir(dir_path: PathBuf) -> Result<()> {
    let mut musics_in_dir: Vec<Music> = vec![];
    let dir = fs::read_dir(&dir_path)?;
    let mut list_pic_base64 = String::new();
    let mut flag = true;
    // 遍历传入文件夹的所有
    for file in dir {
        match file {
            Ok(file_path) => {
                if let Some(ext) = file_path.path().extension() {
                    // 后缀名为mp3
                    if ext.to_str().unwrap() == "mp3" {
                        // println!("Name: {:?}", file_path.path());
                        let tag = Tag::read_from_path(file_path.path())?;
                        let duration = mp3_duration::from_path(file_path.path())?;
                        let picture = tag.pictures().next();
                        #[allow(unused_assignments)]
                        let mut picture_base64 = String::new();
                        match picture {
                            Some(pic_data) => {
                                picture_base64 = format!("data:{};base64,{}",&pic_data.mime_type, BASE64_STANDARD.encode(&pic_data.data));
                            }
                            None => picture_base64 = "".to_string(),
                        }
                        if flag {
                            list_pic_base64.clone_from(&picture_base64);
                            flag = false;
                        }
                        musics_in_dir.push(Music {
                            title: tag.title().map(String::from),
                            artist: tag.artist().map(String::from),
                            album: tag.album().map(String::from),
                            duration: duration.as_secs() as u32,
                            file_path: file_path.path(),
                            picture_base64,
                        })
                    }
                } else {
                    continue;
                }
            }
            Err(e) => return Err(anyhow!(e)),
        }
    }
    // println!("{:?}", musics_in_dir);
    let mut lists_path = utils::get_lists_path()?;
    lists_path.push(PathBuf::from(format!(
        "{}.json",
        dir_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned()
    )));
    let musics_json_path = lists_path;
    // println!("{:?}", musics_json_path);
    // 写入json
    let mut musics_json_file = File::create(musics_json_path)?;
    musics_json_file.write_all(&serde_json::to_vec(&musics_in_dir)?)?;
    // 更新list json
    let list_json_path = utils::get_list_json_path()?;
    // 尝试打开文件，不存在则创建新文件
    let mut music_list: Vec<List> = vec![];
    let mut list_json_file = match File::open(&list_json_path) {
        Err(_) => {
            File::create(&list_json_path)?
        },
        Ok(mut file) => {
            let mut music_list_string = String::new();
            file.read_to_string(&mut music_list_string)?;
            let mut ml: Vec<List> = serde_json::from_str(&music_list_string)?;
            music_list.append(&mut ml);
            File::create(&list_json_path)?
        }
    };
    // 添加新的
    music_list.push(List {
        title: dir_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned(),
        musics_num: musics_in_dir.len(),
        create_at: Local::now().timestamp(),
        folder_path: String::from(dir_path.to_str().unwrap_or_default()),
        picture_base64: list_pic_base64,
    });
    // println!("{:?}", music_list);
    list_json_file.write_all(&serde_json::to_vec(&music_list)?)?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;

    use tauri::api::path;
    use tauri::Config;
    // use id3::{Tag, TagLike};

    #[test]
    fn test_path() {
        let config = Config::default();
        let data_dir = path::app_data_dir(&config);
        println!("{:?}", data_dir);
        // /Users/cakeal/Library/Application Support/Arc Music
    }

    #[test]
    fn test_read_files() {
        let path = PathBuf::from(r"/Users/cakeal/Downloads/喫茶紅魔館 (KNTH-0023)");
        let result = add_dir(path).unwrap();
        // println!("{:?}", result);
        assert_eq!((), result);
    }

    // #[test]
    // fn test_get_picture() {
    //     let tag = Tag::read_from_path(PathBuf::from("/Users/cakeal/Downloads/東方獣王園～Unfinished Dream of All Living Ghost./上海アリス幻樂団 - 強欲な獣のメメント.mp3")).unwrap();
    //     let picture = tag.pictures().next();
    //     match picture {
    //         Some(picture) => {
    //             // println!("{:?}", picture.data.len());

    //         },
    //         None => {
    //             println!("None");
    //         }
    //     }
    // }
}
