use std::process::Command;

use tauri::Window;
use usopp::{dto::{Application, StorageData, SearchResultPayLoad}, utils::{search_apps, search_folders}, config::MAX_LIST_SIZE};

// 根据输入的字符串搜索应用程序
// 暂时不考虑中文搜索、MacOs及Linux
#[tauri::command]
pub fn search(name: &str) -> Result<StorageData ,Vec<Application>> {
    let mut result: Vec<SearchResultPayLoad> = Vec::new();
    // 获取应用程序
    let apps = search_apps(name);
    let folders = search_folders(name);

    if !apps.is_empty() {
        result.extend(apps);
    }

    if !folders.is_empty() {
        result.extend(folders);
    }

    let limited_result = result.into_iter().take(MAX_LIST_SIZE).collect::<Vec<SearchResultPayLoad>>();

    Ok(StorageData {
        data: serde_json::to_value(limited_result).unwrap(),
        status: true,
    })
}

#[tauri::command]
pub fn open(window: Window, r_type: &str, path: &str) {
    match r_type {
       "Application" => {
            Command::new(path)
            .spawn()
            .expect("Failed to open application");
        },
       "Folder" => {
            let mut command = Command::new("explorer");
            command.arg(path);
            command
                .spawn()
                .expect("Failed to open folder");
            },
        _ => {}
    }
}


#[tauri::command]
pub fn window_change(window: Window, event: String) {
  match event.as_str() {
       "blur" => window.hide().unwrap(),
       "focus" => {
        window.show().unwrap();
        let _ = window.set_focus();
       },
       _ => {}
   }
}   
