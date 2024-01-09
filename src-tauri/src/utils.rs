use std::{path::{Path, PathBuf}, fs, io};

use pinyin::{to_pinyin_vec, Pinyin};

use crate::{dto::{Application, SearchResultPayLoad, FolderInfo}, icons::{get_icon, get_icon_bigmap, get_bitmap_buffer, save_icon_file}, storage::read_data, config::{STORAGE_APPS_KEY, STORAGE_FOLDERS_KEY}, enums::SearchPayLoadEvent};

// 获取应用程序
pub fn search_apps(name: &str) -> Vec<SearchResultPayLoad> {
  let py_name = get_pin_yin(name);
  let mut apps: Vec<Application> = vec![];
  let result = read_data(STORAGE_APPS_KEY);
  match result {
      Ok(res) => {
        apps = res.data.as_array().expect("Invalid data format")
        .iter()
        .map(|app| {
            serde_json::from_value::<Application>(app.clone()).expect("Failed to deserialize")
        })
        .collect();
      }
      Err(e) => {
          println!("{}", e);
      }
  }
 
  let filtered_apps: Vec<&Application> = apps
  .iter()
  .filter(|app| app.soft_name.to_lowercase().replace(" ", "").contains(&name) || get_pin_yin(&app.soft_name.to_lowercase().replace(" ", "")).contains(&py_name))
  .collect();
  let mut apps_payload: Vec<SearchResultPayLoad> = Vec::new();

  filtered_apps.iter().for_each(|item| {
    if !item.soft_uninstall_path.is_empty() {
      let un_path = &item.soft_uninstall_path.trim_matches('"').to_owned();
      let un_path = un_path.replace("/allusers", "");
      let parent_dir = Path::new(&un_path).parent().expect("Failed to get parent folder");
      let exe_path_list = get_folder_exe(parent_dir.to_str().unwrap(), item.soft_name.clone(), item.soft_icon_name.clone());
      exe_path_list.iter().for_each(|exe_path| {
        apps_payload.push(get_app_info(exe_path, &item));
      });
    }
  });
  apps_payload
}

// 获取文件夹
pub fn search_folders(name: &str) -> Vec<SearchResultPayLoad> {
  let mut folders: Vec<FolderInfo> = vec![];
  let result = read_data(STORAGE_FOLDERS_KEY);
  match result {
      Ok(res) => {
        folders = res.data.as_array().expect("Invalid data format")
        .iter()
        .map(|app| {
            serde_json::from_value::<FolderInfo>(app.clone()).expect("Failed to deserialize")
        })
        .collect();
      }
      Err(e) => {
          println!("{}", e);
      }
  }
  let filtered_folders: Vec<&FolderInfo> = folders
  .iter()
  .filter(|app| app.name.to_lowercase().replace(" ", "").contains(&name))
  .collect();

  let mut folder_icon_path = String::new();

  if !filtered_folders.is_empty() {
    let folder_path = filtered_folders[0].path.clone();
    if let Some(icon) = get_icon(&folder_path) {
      if let Some(map) = get_icon_bigmap(icon) {
        if let Some(buffer) = get_bitmap_buffer(map) {
          folder_icon_path = save_icon_file(&buffer, "folder");
        
        }
      }
    }
  }

  let mut folder_payload: Vec<SearchResultPayLoad> = vec![];
  filtered_folders.iter().for_each(|item| {
    folder_payload.push(SearchResultPayLoad {
      name: item.name.clone(),
      text_name: item.name.clone(),
      // r_type: SearchPayLoadEvent::Folder,
      r_type: "Folder".to_string(),
      r_publisher: None,
      r_version: None,
      r_main_pro_path: None,
      r_exe_path: Some(item.path.clone()),
      r_icon_path: Some(folder_icon_path.clone()),
    })
  });
  folder_payload
}

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

// 获取目录下文件夹信息 folder_path 目录 max_depth 目录深度 current_depth 当前目录深度
pub fn get_folder_info_recursive(folder_path: String, max_depth: u32, current_depth: u32) -> Vec<FolderInfo> {
  let mut folder_list: Vec<FolderInfo> = Vec::new();

  if current_depth > max_depth {
    return folder_list;
  }

  if let Ok(entries) = fs::read_dir(folder_path) {
    for entry in entries {
      if let Ok(entry) = entry {
        if let Ok(file_type) = entry.file_type() {
          if file_type.is_dir() {
            let folder_name = entry.file_name().to_string_lossy().to_string();
            let folder_path = entry.path();
            // 跳过隐藏目录
            if folder_name.starts_with(".") {
              continue;
            }
            let folder_info = FolderInfo {
              name: folder_name.clone(),
              path: folder_path.clone().to_string_lossy().to_string(),
            };
            folder_list.push(folder_info);
            let subdirectories = get_folder_info_recursive(folder_path.to_string_lossy().to_string(), max_depth, current_depth + 1);
            folder_list.extend(subdirectories);
          }
        }
      }
    }
  }
  folder_list
}

// 注册表内没有主程序的exe文件, 很多应用程序的主程序名称未知 所以干脆获取目录下所有exe
// 未来有更好的思路进行更改
fn get_folder_exe(dir_path: &str, app_name: String, icon_name: String) -> Vec<String> {
  let mut list: Vec<String> = Vec::new();
  let mut is_match_app_name = false;
  let mut match_app_path = String::new();
  if let Ok(entries) = fs::read_dir(dir_path) {

    for entry in entries {
      if let Ok(entry) = entry {
        let file_path = entry.path();
        let path = Path::new(&file_path);
        let file_name = path.file_stem().unwrap_or_default();
        let mut file_name_without_extension = PathBuf::new();
        file_name_without_extension.set_file_name(file_name);
        let file_name_without_extension = file_name_without_extension.to_string_lossy().into_owned();

        let parent_dir_name = match path.parent() {
            Some(parent_dir) => parent_dir.file_name().unwrap_or_default().to_string_lossy().into_owned(),
            None => String::new(),
        };
        
        if file_path.is_file() && file_path.extension().unwrap_or_default() == "exe" {
          list.push(file_path.to_str().unwrap().to_owned());
          if icon_name.starts_with(&file_name_without_extension) || app_name.starts_with(&file_name_without_extension) || file_name_without_extension == parent_dir_name {
           
            is_match_app_name = true;
            match_app_path = file_path.to_str().unwrap().to_owned();
            
          }
        }
      }
    }
  }
  // 应用name、或者父级路径 如 d/bilibili/xx.exe 能够匹配上[name].exe 那就直接返回
  if is_match_app_name {
    list.clear();
    list.push(match_app_path)
  }
  list
}

// 通过exe路径 获取应用信息
fn get_app_info(path: &str, app: &Application) -> SearchResultPayLoad {
  let mut pay_load = SearchResultPayLoad {
      name: app.name.clone(),
      text_name: app.soft_name.clone(),
      r_publisher: Some(app.soft_publisher.clone()),
      r_version: Some(app.soft_version.clone()),
      r_exe_path: Some(path.to_owned()),
      r_main_pro_path: Some(app.soft_main_pro_path.clone()),
      r_icon_path: None,
      r_type: "Application".to_string()
      // soft_icon_buffer: vec![],
  };
  
  if let Some(icon) = get_icon(&path) {
    if let Some(map) = get_icon_bigmap(icon) {
      if let Some(buffer) = get_bitmap_buffer(map) {
        pay_load.r_icon_path = Some(save_icon_file(&buffer, &app.soft_name));
        // pay_load.soft_icon_buffer = buffer;
      }
    }
  }
  pay_load
}


// 获取盘符
pub fn get_logical_drive_letters() ->  Vec<char> {
  let mut drive_letters: Vec<char> = Vec::new();

  for letter in b'A'..=b'Z' {
      drive_letters.push(letter as char);
  }

  drive_letters
}

pub fn check_drive_exists(drive_letter: char) -> bool {
  let drive_path = format!("{}:\\", drive_letter);

  if let Ok(metadata) = fs::metadata(drive_path) {
      metadata.is_dir()
  } else {
      false
  }
}
