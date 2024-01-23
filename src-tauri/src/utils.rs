use std::{path::Path, fs};

use pinyin::{to_pinyin_vec, Pinyin};
use tauri::PhysicalPosition;

// 创建目录
pub fn create_folder(dir_name: &str) {
  let path = Path::new(&dir_name); // 将字符串转换为路径对象
  
  if !path.exists() { // 判断路径是否已经存在
      fs::create_dir(path).expect("无法创建目录"); // 创建新的文件夹
  }
}

// 获取拼音
pub fn get_pin_yin(parma: &str) -> String {
    let mut pin_yin = String::new();
    for char_item in parma.to_string().chars(){
        // 获取转换的拼音
        let transform_pinyin = to_pinyin_vec(char_item.to_string().as_str(), Pinyin::plain).join("");
        let mut transform_pinyin_str = transform_pinyin.as_str();
        let mut temp: String = String::new();
        if transform_pinyin_str ==  ""&& char_item.to_string() != " " {
            temp = char_item.to_string().to_lowercase();
            transform_pinyin_str= temp.as_str();
        }
        pin_yin.push_str(transform_pinyin_str);
    } 
  pin_yin
}

// 获取盘符
pub fn get_logical_drive_letters() ->  Vec<char> {
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

// 获取窗口位置
pub fn get_window_position(window: &tauri::Window) -> PhysicalPosition<i32> {

  if let Ok(position)  = window.inner_position() {
    return position;
  }
 
  PhysicalPosition::new(0, 0)
}
