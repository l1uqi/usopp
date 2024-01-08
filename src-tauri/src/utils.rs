use std::{path::{Path, PathBuf}, fs};

use pinyin::{to_pinyin_vec, Pinyin};

use crate::{dto::{Application, ApplicationPayLoad}, icons::{get_icon, get_icon_bigmap, get_bitmap_buffer, save_icon_file}};

pub fn get_apps(apps: &Vec<&Application>) -> Vec<ApplicationPayLoad> {
  let mut apps_payload: Vec<ApplicationPayLoad> = Vec::new();
  apps.iter().for_each(|item| {
    if !item.soft_uninstall_path.is_empty() {
      let un_path = &item.soft_uninstall_path.trim_matches('"').to_owned();
      let un_path = un_path.replace("/allusers", "");
      let parent_dir = Path::new(&un_path).parent().expect("Failed to get parent directory");
      let exe_path_list = get_directory_exe(parent_dir.to_str().unwrap(), item.soft_name.clone());
      exe_path_list.iter().for_each(|exe_path| {
        apps_payload.push(get_app_info(exe_path, &item));
      });
    }
  });
  apps_payload
}

// 注册表内没有主程序的exe文件, 很多应用程序的主程序名称未知 所以干脆获取目录下所有exe
// 未来有更好的思路进行更改
fn get_directory_exe(dir_path: &str, app_name: String) -> Vec<String> {
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
          if app_name.starts_with(&file_name_without_extension) || file_name_without_extension == parent_dir_name {
           
            is_match_app_name = true;
            match_app_path = file_path.to_str().unwrap().to_owned();
            
          }
        }
      }
    }
  }
  if is_match_app_name {
    list.clear();
    list.push(match_app_path)
  }
  list
}

// 通过exe路径 获取应用信息
fn get_app_info(path: &str, app: &Application) -> ApplicationPayLoad {
  let mut pay_load = ApplicationPayLoad {
      name: app.name.clone(),
      soft_name: app.soft_name.clone(),
      soft_name_init: app.soft_name_init.clone(),
      soft_publisher: app.soft_publisher.clone(),
      soft_version: app.soft_version.clone(),
      soft_run_path: path.to_owned(),
      soft_icon_path: "".to_string(),
      soft_icon_buffer: vec![],
  };
  
  if let Some(icon) = get_icon(&path) {
    if let Some(map) = get_icon_bigmap(icon) {
      if let Some(buffer) = get_bitmap_buffer(map) {
        pay_load.soft_icon_path = save_icon_file(&buffer, &app.soft_name);
        pay_load.soft_icon_buffer = buffer;
      }
    }
  }
  pay_load
}

pub fn create_folder(dir_name: &str) {
  let path = Path::new(&dir_name); // 将字符串转换为路径对象
  
  if !path.exists() { // 判断路径是否已经存在
      fs::create_dir(path).expect("无法创建目录"); // 创建新的文件夹
  }
}
pub fn get_pin_yin(parma: & str) -> String {
  let a = to_pinyin_vec(parma, Pinyin::plain).join("");
  let mut b = a.as_str();
  let mut temp: String = String::new();
  if b == "" {
      temp = parma.to_lowercase();
      b = temp.as_str();
  }
  b.to_string()
}