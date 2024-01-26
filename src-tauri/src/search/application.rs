use std::{fs, path::{Path, PathBuf}};

use winreg::{RegKey, enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, KEY_READ}};

use crate::{config::{UNINSTALL_KEY, UNINSTALL_KEY_2}, dto::{Application, FileType, SearchResult}, icons::{get_bitmap_buffer, get_icon, get_icon_bigmap, save_icon_file}, utils::get_pin_yin};

pub fn search_applications_by_name(name: &str) -> Vec<SearchResult> {
  let app_list: Vec<Application> = get_application_list();
  filter_applications_by_name(&name, &app_list)
}

// 获取本地应用列表
fn get_application_list() -> Vec<Application> {
  let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
  let hklm_2 = RegKey::predef(HKEY_CURRENT_USER);

  let uninstall_key: RegKey = hklm.open_subkey_with_flags(UNINSTALL_KEY, KEY_READ).unwrap();
  let uninstall_key_2: RegKey = hklm.open_subkey_with_flags(UNINSTALL_KEY_2, KEY_READ).unwrap();
  let uninstall_key_3: RegKey = hklm_2.open_subkey_with_flags(UNINSTALL_KEY_2, KEY_READ).unwrap();

  let subkeys = get_subkeys(&uninstall_key);
  let subkeys_2 = get_subkeys(&uninstall_key_2);
  let subkeys_3: Vec<String> = get_subkeys(&uninstall_key_3);

  let mut applications: Vec<Application> = vec![];

  for subkey in subkeys {
      if let Some(application) = get_application_info(&uninstall_key, &subkey) {
        applications.push(application);
      }
  }

  for subkey in subkeys_2 {
    if let Some(application) = get_application_info(&uninstall_key_2, &subkey) {
      applications.push(application);
    }
  }

  for subkey in subkeys_3 {
    if let Some(application) = get_application_info(&uninstall_key_3, &subkey) {
      applications.push(application);
    }
  }
  applications
}

// 获取本地应用信息
fn get_application_info(reg_key: &RegKey, subkey: &str) -> Option<Application> {
  let app_key = reg_key.open_subkey_with_flags(subkey, KEY_READ).ok()?;
  let sys_component: String = app_key.get_value("SystemComponent").unwrap_or_default();
  let soft_name: String = app_key.get_value("DisplayName").unwrap_or_default();
  let soft_ico: String = app_key.get_value("DisplayIcon").unwrap_or_default();
  let soft_parent_key: String = app_key.get_value("ParentKeyName").unwrap_or_default();
  let soft_parent_display_name: String = app_key.get_value("ParentDisplayName").unwrap_or_default();
  if !soft_name.is_empty() && sys_component!= "1" && soft_parent_key.is_empty() && soft_parent_display_name.is_empty() {
    let soft_publisher: String = app_key.get_value("Publisher").unwrap_or_default();
    let soft_version: String = app_key.get_value("DisplayVersion").unwrap_or_default();
    let soft_main_pro_path: String = app_key.get_value("InstallLocation").unwrap_or_default();
    let soft_uninstall_path: String = app_key.get_value("UninstallString").unwrap_or_default();
    let soft_size: u64 = app_key.get_value("EstimatedSize").unwrap_or_default();
    let soft_icon_path: &Path = Path::new(&soft_ico);
    let mut soft_icon_name = String::new();
    if soft_icon_path.is_file() {
      let icon_filename = Path::new(&soft_icon_path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("");
      soft_icon_name = icon_filename
        .rsplitn(2, '.')
        .nth(1)
        .unwrap_or("")
        .to_lowercase();
    }
    Some(Application {
      name: subkey.to_owned(),
      soft_name,
      soft_icon_name,
      soft_publisher,
      soft_version,
      soft_main_pro_path,
      soft_uninstall_path,
      soft_size,
    })
  } else {
    None
  }
}

// 获取应用程序
pub fn filter_applications_by_name(name: &str, apps: &Vec<Application>) -> Vec<SearchResult> {
  let py_name = get_pin_yin(name);
 
  let filtered_apps: Vec<&Application> = apps
  .iter()
  .filter(|app| app.soft_name.to_lowercase().replace(" ", "").contains(&name) || get_pin_yin(&app.soft_name.to_lowercase().replace(" ", "")).contains(&py_name))
  .collect();
  let mut apps_payload: Vec<SearchResult> = Vec::new();

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

fn get_subkeys(reg_key: &RegKey) -> Vec<String> {
  reg_key.enum_keys().map(|x| x.unwrap()).collect()
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
fn get_app_info(path: &str, app: &Application) -> SearchResult {
  let mut pay_load = SearchResult {
      name: app.name.clone(),
      text_name: app.soft_name.clone(),
      r_publisher: Some(app.soft_publisher.clone()),
      r_version: Some(app.soft_version.clone()),
      r_exe_path: Some(path.to_owned()),
      r_main_pro_path: Some(app.soft_main_pro_path.clone()),
      r_icon_path: None,
      r_type: FileType::Application
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
