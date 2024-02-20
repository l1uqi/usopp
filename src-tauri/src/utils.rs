use std::{
    fs,
    path::{Path, PathBuf}, sync::MutexGuard,
};

use pinyin::{to_pinyin_vec, Pinyin};
use strsim::levenshtein;
use tauri::PhysicalPosition;

use crate::{
    config::MAX_LIST_SIZE, database::IndexDatabase, dto::{FileEntry, FileType}, icons::{get_bitmap_buffer, get_icon, get_icon_bigmap, save_icon_file}
};

// 创建目录
pub fn create_folder(dir_name: &str) {
    let path = Path::new(&dir_name); // 将字符串转换为路径对象

    if !path.exists() {
        // 判断路径是否已经存在
        fs::create_dir(path).expect("无法创建目录"); // 创建新的文件夹
    }
}

// 获取拼音
pub fn get_pin_yin(parma: &str) -> String {
    let mut pin_yin = String::new();
    for char_item in parma.to_string().chars() {
        // 获取转换的拼音
        let transform_pinyin =
            to_pinyin_vec(char_item.to_string().as_str(), Pinyin::plain).join("");
        let mut transform_pinyin_str = transform_pinyin.as_str();
        let mut temp: String = String::new();
        if transform_pinyin_str == "" && char_item.to_string() != " " {
            temp = char_item.to_string().to_lowercase();
            transform_pinyin_str = temp.as_str();
        }
        pin_yin.push_str(transform_pinyin_str);
    }
    pin_yin
}

// 获取盘符
pub fn get_logical_drive_letters() -> Vec<char> {
    let mut drive_letters: Vec<char> = Vec::new();

    for letter in b'A'..=b'Z' {
        if check_drive_exists(letter as char) {
            drive_letters.push(letter as char);
        }
    }

    drive_letters
}

// 判断盘符是否存在
pub fn check_drive_exists(drive_letter: char) -> bool {
    let drive_path = format!("{}:\\", drive_letter);

    if let Ok(metadata) = fs::metadata(drive_path) {
        metadata.is_dir()
    } else {
        false
    }
}

// 判断文件是否存在
pub fn check_path_file(file_path: &str) -> bool {
    let path = Path::new(&file_path); // 将字符串转换为路径对象

    path.exists()
}

// 获取窗口位置
pub fn get_window_position(window: &tauri::Window) -> PhysicalPosition<i32> {
    if let Ok(position) = window.inner_position() {
        return position;
    }
    PhysicalPosition::new(0, 0)
}

// 获取排序后结果
pub fn get_sorted_result(result: Vec<FileEntry>, name: &str) -> Vec<FileEntry> {
    let result: Vec<FileEntry> = result
        .clone()
        .into_iter()
        .map(|mut result| {
            let path_buf = PathBuf::from(&result.path);
                let file_type = match path_buf.extension().and_then(|ext| ext.to_str()) {
                    Some("exe") => FileType::Application,
                    Some("lnk") => FileType::LNK,
                    Some("txt") => FileType::TXT,
                    Some("pdf") => FileType::PDF,
                    Some("zip") => FileType::ZIP,
                    Some("jpg") => FileType::JPG,
                    Some("gif") => FileType::GIF,
                    Some("png") => FileType::PNG,
                    Some("js") => FileType::JS,
                    Some("xlsx") => FileType::XLSX,
                    Some("ppt") => FileType::PPT,
                    Some("pptx") => FileType::PPT,
                    Some("doc") => FileType::DOC,
                    _ => {
                        if path_buf.is_dir() {
                            FileType::Directory
                        } else {
                            // Handle other unknown file types
                            FileType::File
                        }
                    }
                };
                let icon_path = match file_type {
                    FileType::Application => Some(get_file_icon(&result.path, &result.name)),
                    FileType::LNK => Some(get_file_icon(&result.path, &result.name)),
                    _ => None,
                };
                result.file_type = file_type;
                result.icon_path = icon_path;
            result
        })
        .collect();
    // 根据 Levenshtein 距离排序结果
    let mut sorted_results: Vec<(FileEntry, usize)> = result
        .into_iter()
        .map(|result| (result.clone(), levenshtein(&result.name, name)))
        .collect();

    sorted_results.sort_by(|(result1, dist1), (result2, dist2)| {
        match (result1.file_type, result2.file_type) {
            (FileType::Application, FileType::Application) => dist1.cmp(dist2),
            (FileType::Application, _) => std::cmp::Ordering::Less,
            (_, FileType::Application) => std::cmp::Ordering::Greater,
            _ => dist1.cmp(dist2),
        }
    });

    // 提取匹配度最高的前 50 条结果
    // 添加文件类型、图标
    let result = sorted_results
        .into_iter()
        .take(MAX_LIST_SIZE)
        .map(|(result, _)| result)
        .collect();

    result
}

// 文件不存在, 则移除索引
pub fn remove_file_index(result: Vec<FileEntry>, mut database: MutexGuard<'_, IndexDatabase>) -> Vec<FileEntry> {
   let list: Vec<FileEntry> = result.into_iter().filter(|data| {
       let is = check_path_file(&data.path);
       if !is {
        database.delete(&data.path)
       }
       is
   }).collect();
   list
}

fn get_file_icon(f_path: &str, f_name: &str) -> String {
    let mut icon_path = String::new();
    if let Some(icon) = get_icon(f_path) {
        if let Some(map) = get_icon_bigmap(icon) {
            if let Some(buffer) = get_bitmap_buffer(map) {
                icon_path = save_icon_file(&buffer, f_name);
            }
        }
    }
    icon_path
}
