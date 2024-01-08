use std::process::Command;

use tauri::{Window, Manager};
use usopp::{storage::read_data, dto::{Application, StorageData}, utils::{get_apps, get_pin_yin}};

// 根据输入的字符串搜索应用程序
// 暂时不考虑中文搜索、MacOs及Linux
#[tauri::command]
pub fn search(name: &str) -> Result<StorageData ,Vec<Application>> {
    let name = get_pin_yin(name);
    let mut apps: Vec<Application> = vec![];
    let result = read_data("apps");
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
    .filter(|app| app.soft_name.to_lowercase().replace(" ", "").contains(&name))
    .collect();
    let apps = get_apps(&filtered_apps);
    Ok(StorageData {
        data: serde_json::to_value(apps).unwrap(),
        status: true,
    })
}

#[tauri::command]
pub fn open(window: Window, app_path: &str) {
   Command::new(app_path)
        .spawn()
        .expect("Failed to open application");
    window.hide().unwrap()
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
